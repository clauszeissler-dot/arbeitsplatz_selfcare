use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Persistence {
    dir: PathBuf,
    next_temp: Arc<AtomicU64>,
}

pub struct LoadResult<T> {
    pub value: T,
    pub warning: Option<String>,
}

pub struct Persisted<R> {
    pub value: R,
    pub result: Result<(), String>,
}

impl Persistence {
    pub fn open(dir: PathBuf) -> Result<Self, String> {
        secure_directory(&dir)?;
        Ok(Self {
            dir,
            next_temp: Arc::new(AtomicU64::new(0)),
        })
    }

    pub fn load<T: DeserializeOwned + Default>(&self, name: &str) -> Result<LoadResult<T>, String> {
        validate_component(name)?;
        let path = self.dir.join(name);
        let Some(metadata) = safe_file_metadata(&path)? else {
            return Ok(LoadResult {
                value: T::default(),
                warning: None,
            });
        };
        if !metadata.file_type().is_file() {
            return Err(format!("{} ist keine reguläre Datei", path.display()));
        }
        set_file_mode(&path, 0o600)?;
        let bytes = fs::read(&path)
            .map_err(|error| format!("{} konnte nicht gelesen werden: {error}", path.display()))?;
        match serde_json::from_slice(&bytes) {
            Ok(value) => Ok(LoadResult {
                value,
                warning: None,
            }),
            Err(error) => {
                let corrupt = self.dir.join(format!("{name}.corrupt"));
                remove_previous_quarantine(&corrupt)?;
                fs::rename(&path, &corrupt).map_err(|rename_error| {
                    format!(
                        "Beschädigte Datei {} konnte nicht gesichert werden: {rename_error}",
                        path.display()
                    )
                })?;
                set_file_mode(&corrupt, 0o600)?;
                sync_directory(&self.dir)?;
                Ok(LoadResult {
                    value: T::default(),
                    warning: Some(format!(
                        "{name} war beschädigt und wurde als {} gesichert: {error}",
                        corrupt.file_name().unwrap_or_default().to_string_lossy()
                    )),
                })
            }
        }
    }

    pub fn save<T: Serialize + ?Sized>(&self, name: &str, value: &T) -> Result<(), String> {
        let mut bytes = serde_json::to_vec_pretty(value)
            .map_err(|error| format!("{name} konnte nicht serialisiert werden: {error}"))?;
        bytes.push(b'\n');
        self.write_atomic(&self.dir, name, &bytes).map(|_| ())
    }

    pub fn write_private(
        &self,
        directory: &str,
        name: &str,
        bytes: &[u8],
    ) -> Result<PathBuf, String> {
        validate_component(directory)?;
        validate_component(name)?;
        let dir = self.dir.join(directory);
        secure_directory(&dir)?;
        self.write_atomic(&dir, name, bytes)
    }

    fn write_atomic(&self, dir: &Path, name: &str, bytes: &[u8]) -> Result<PathBuf, String> {
        validate_component(name)?;
        let target = dir.join(name);
        if let Some(metadata) = safe_file_metadata(&target)? {
            if !metadata.file_type().is_file() {
                return Err(format!("{} ist keine reguläre Datei", target.display()));
            }
        }
        let sequence = self.next_temp.fetch_add(1, Ordering::Relaxed);
        let temp = dir.join(format!(".{name}.tmp.{}.{sequence}", std::process::id()));
        let result = (|| {
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(&temp).map_err(|error| {
                format!(
                    "Temporäre Datei {} konnte nicht angelegt werden: {error}",
                    temp.display()
                )
            })?;
            file.write_all(bytes).map_err(|error| {
                format!(
                    "Temporäre Datei {} konnte nicht geschrieben werden: {error}",
                    temp.display()
                )
            })?;
            file.sync_all().map_err(|error| {
                format!(
                    "Temporäre Datei {} konnte nicht synchronisiert werden: {error}",
                    temp.display()
                )
            })?;
            drop(file);
            fs::rename(&temp, &target).map_err(|error| {
                format!(
                    "{} konnte nicht atomar ersetzt werden: {error}",
                    target.display()
                )
            })?;
            set_file_mode(&target, 0o600)?;
            sync_directory(dir)?;
            Ok(target.clone())
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temp);
        }
        result
    }
}

pub fn mutate_and_save<T, P, R>(
    lock: &Mutex<T>,
    persistence: &Persistence,
    name: &str,
    mutate: impl FnOnce(&mut T) -> R,
    persisted: impl Fn(&T) -> &P,
) -> Persisted<R>
where
    P: Serialize + ?Sized,
{
    let mut guard = lock.lock().unwrap();
    let value = mutate(&mut guard);
    let result = persistence.save(name, persisted(&guard));
    Persisted { value, result }
}

fn validate_component(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value == "."
        || value == ".."
        || value.contains('/')
        || value.contains('\\')
    {
        return Err(format!("Ungültiger Dateiname: {value:?}"));
    }
    Ok(())
}

fn safe_file_metadata(path: &Path) -> Result<Option<fs::Metadata>, String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            Err(format!("{} ist ein symbolischer Link", path.display()))
        }
        Ok(metadata) => Ok(Some(metadata)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!(
            "{} konnte nicht geprüft werden: {error}",
            path.display()
        )),
    }
}

fn secure_directory(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(format!("{} ist ein symbolischer Link", path.display()));
        }
        Ok(metadata) if !metadata.is_dir() => {
            return Err(format!("{} ist kein Verzeichnis", path.display()));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir_all(path).map_err(|create_error| {
                format!(
                    "{} konnte nicht angelegt werden: {create_error}",
                    path.display()
                )
            })?;
        }
        Err(error) => {
            return Err(format!(
                "{} konnte nicht geprüft werden: {error}",
                path.display()
            ))
        }
    }
    set_file_mode(path, 0o700)
}

fn remove_previous_quarantine(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {
            Err(format!("{} ist ein Verzeichnis", path.display()))
        }
        Ok(_) => fs::remove_file(path).map_err(|error| {
            format!(
                "Alte Diagnosekopie {} konnte nicht entfernt werden: {error}",
                path.display()
            )
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "{} konnte nicht geprüft werden: {error}",
            path.display()
        )),
    }
}

fn sync_directory(path: &Path) -> Result<(), String> {
    File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| {
            format!(
                "Verzeichnis {} konnte nicht synchronisiert werden: {error}",
                path.display()
            )
        })
}

#[cfg(unix)]
fn set_file_mode(path: &Path, mode: u32) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).map_err(|error| {
        format!(
            "Rechte für {} konnten nicht gesetzt werden: {error}",
            path.display()
        )
    })
}

#[cfg(not(unix))]
fn set_file_mode(_path: &Path, _mode: u32) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
    struct Data {
        value: u64,
    }

    struct TestDir(PathBuf);

    impl TestDir {
        fn new(label: &str) -> Self {
            static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let path =
                std::env::temp_dir().join(format!("lotse-{label}-{}-{n}", std::process::id()));
            let _ = fs::remove_dir_all(&path);
            Self(path)
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[cfg(unix)]
    fn mode(path: &Path) -> u32 {
        use std::os::unix::fs::PermissionsExt;
        fs::metadata(path).unwrap().permissions().mode() & 0o777
    }

    #[test]
    #[cfg(unix)]
    fn creates_owner_only_directory_and_json_file() {
        let test = TestDir::new("permissions");
        let persistence = Persistence::open(test.0.clone()).unwrap();
        persistence.save("state.json", &Data { value: 7 }).unwrap();
        assert_eq!(mode(&test.0), 0o700);
        assert_eq!(mode(&test.0.join("state.json")), 0o600);
        assert_eq!(
            persistence.load::<Data>("state.json").unwrap().value,
            Data { value: 7 }
        );
        assert!(fs::read_dir(&test.0).unwrap().all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .contains(".tmp.")));
    }

    #[test]
    #[cfg(unix)]
    fn repairs_existing_permissions() {
        use std::os::unix::fs::PermissionsExt;
        let test = TestDir::new("repair");
        fs::create_dir_all(&test.0).unwrap();
        fs::set_permissions(&test.0, fs::Permissions::from_mode(0o755)).unwrap();
        fs::write(test.0.join("settings.json"), r#"{"value":3}"#).unwrap();
        fs::set_permissions(
            test.0.join("settings.json"),
            fs::Permissions::from_mode(0o644),
        )
        .unwrap();
        let persistence = Persistence::open(test.0.clone()).unwrap();
        let loaded = persistence.load::<Data>("settings.json").unwrap();
        assert_eq!(loaded.value.value, 3);
        assert_eq!(mode(&test.0), 0o700);
        assert_eq!(mode(&test.0.join("settings.json")), 0o600);
    }

    #[test]
    #[cfg(unix)]
    fn refuses_to_write_through_a_symlink() {
        use std::os::unix::fs::symlink;
        let test = TestDir::new("symlink");
        let outside = test.0.with_extension("outside");
        let persistence = Persistence::open(test.0.clone()).unwrap();
        fs::write(&outside, "unchanged").unwrap();
        symlink(&outside, test.0.join("state.json")).unwrap();
        let error = persistence
            .save("state.json", &Data { value: 9 })
            .unwrap_err();
        assert!(error.contains("symbolischer Link"));
        assert_eq!(fs::read_to_string(&outside).unwrap(), "unchanged");
        let _ = fs::remove_file(outside);
    }

    #[test]
    #[cfg(unix)]
    fn quarantines_corrupt_json_and_keeps_only_one_copy() {
        let test = TestDir::new("corrupt");
        let persistence = Persistence::open(test.0.clone()).unwrap();
        fs::write(test.0.join("state.json"), "not json").unwrap();
        let first = persistence.load::<Data>("state.json").unwrap();
        assert_eq!(first.value, Data::default());
        assert!(first.warning.unwrap().contains("beschädigt"));
        assert!(test.0.join("state.json.corrupt").exists());
        assert_eq!(mode(&test.0.join("state.json.corrupt")), 0o600);
        fs::write(test.0.join("state.json"), "also broken").unwrap();
        let _ = persistence.load::<Data>("state.json").unwrap();
        let copies = fs::read_dir(&test.0)
            .unwrap()
            .filter(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .contains("corrupt")
            })
            .count();
        assert_eq!(copies, 1);
    }

    #[test]
    fn mutation_order_is_the_durable_write_order() {
        let test = TestDir::new("ordering");
        let persistence = Persistence::open(test.0.clone()).unwrap();
        let value = Arc::new(Mutex::new(Data::default()));
        let entered = Arc::new(Barrier::new(2));
        let release = Arc::new(Barrier::new(2));

        let first_value = Arc::clone(&value);
        let first_persistence = persistence.clone();
        let first_entered = Arc::clone(&entered);
        let first_release = Arc::clone(&release);
        let first = thread::spawn(move || {
            mutate_and_save(
                &first_value,
                &first_persistence,
                "state.json",
                |data| {
                    data.value = 1;
                    first_entered.wait();
                    first_release.wait();
                },
                |data| data,
            )
            .result
            .unwrap();
        });

        entered.wait();
        let second_value = Arc::clone(&value);
        let second_persistence = persistence.clone();
        let second = thread::spawn(move || {
            mutate_and_save(
                &second_value,
                &second_persistence,
                "state.json",
                |data| data.value = 2,
                |data| data,
            )
            .result
            .unwrap();
        });
        release.wait();
        first.join().unwrap();
        second.join().unwrap();

        assert_eq!(
            persistence.load::<Data>("state.json").unwrap().value.value,
            2
        );
    }
}
