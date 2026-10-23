mod collect;
#[cfg(target_os = "macos")]
mod macos_av;

use lotse_core::engine::State as EngineState;
use lotse_core::{Engine, Hint, HintKind, Level, Posture, Settings, Snapshot};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;
use tauri::image::Image;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, LogicalPosition, Manager, RunEvent, WindowEvent};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

const TICK: Duration = Duration::from_secs(20);
const TOAST_W: f64 = 380.0;
const TOAST_MARGIN: f64 = 14.0;

struct Shared {
    engine: Mutex<Engine>,
    dir: PathBuf,
    status_item: Mutex<Option<MenuItem<tauri::Wry>>>,
}

fn now() -> i64 {
    chrono::Local::now().timestamp()
}

fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn load<T: serde::de::DeserializeOwned + Default>(path: &PathBuf) -> T {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save<T: serde::Serialize>(path: &PathBuf, value: &T) {
    if let Ok(json) = serde_json::to_string_pretty(value) {
        let tmp = path.with_extension("tmp");
        if std::fs::write(&tmp, json).is_ok() {
            let _ = std::fs::rename(tmp, path);
        }
    }
}

// ---------- Tray ----------

fn level_rgb(level: Level) -> [u8; 3] {
    match level {
        Level::Green => [0x22, 0xC5, 0x5E],
        Level::Yellow => [0xF5, 0xB0, 0x0B],
        Level::Red => [0xEF, 0x44, 0x44],
    }
}

/// Runder Ampelpunkt (44×44 px = 22 pt Retina) mit weichem Rand.
fn dot_icon(level: Level, meeting: bool) -> Image<'static> {
    const S: usize = 44;
    let [r, g, b] = level_rgb(level);
    let mut px = vec![0u8; S * S * 4];
    let c = (S as f32 - 1.0) / 2.0;
    let radius = 13.0;
    for y in 0..S {
        for x in 0..S {
            let d = ((x as f32 - c).powi(2) + (y as f32 - c).powi(2)).sqrt();
            let mut a = (radius + 0.5 - d).clamp(0.0, 1.0);
            // Im Meeting: nur Ring, Hinweise ruhen
            if meeting {
                a *= (d - (radius - 4.0) + 0.5).clamp(0.0, 1.0);
            }
            let i = (y * S + x) * 4;
            px[i..i + 4].copy_from_slice(&[r, g, b, (a * 255.0) as u8]);
        }
    }
    Image::new_owned(px, S as u32, S as u32)
}

fn status_line(s: &Snapshot) -> String {
    let lvl = match s.level {
        Level::Green => "Grün",
        Level::Yellow => "Gelb",
        Level::Red => "Rot",
    };
    let mut line = format!(
        "{lvl} · {} min Fokus · {:.1}/{:.1} l · {}",
        s.block_min,
        s.water_ml as f64 / 1000.0,
        s.water_target_ml as f64 / 1000.0,
        s.posture_label
    );
    if s.in_meeting {
        line.push_str(" · Meeting");
    }
    line
}

fn update_tray(app: &AppHandle, snap: &Snapshot) {
    let shared = app.state::<Shared>();
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_icon(Some(dot_icon(snap.level, snap.in_meeting)));
        let title = if snap.in_break { "Pause".to_string() } else { format!("{}′", snap.block_min) };
        let _ = tray.set_title(Some(title));
        let _ = tray.set_tooltip(Some(status_line(snap)));
    }
    let item = shared.status_item.lock().unwrap().clone();
    if let Some(item) = item {
        let _ = item.set_text(status_line(snap));
    }
}

// ---------- Fenster ----------

fn show_dashboard(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

fn show_toast(app: &AppHandle, hint: &Hint) {
    let Some(w) = app.get_webview_window("toast") else { return };
    if let Ok(Some(m)) = w.primary_monitor() {
        let scale = m.scale_factor();
        let size = m.size().to_logical::<f64>(scale);
        let pos = m.position().to_logical::<f64>(scale);
        let _ = w.set_position(LogicalPosition::new(pos.x + size.width - TOAST_W - TOAST_MARGIN, pos.y + 38.0));
    }
    let _ = app.emit_to("toast", "hint", hint);
    let _ = w.show();
}

// ---------- Takt ----------

fn publish(app: &AppHandle, snap: &Snapshot) {
    update_tray(app, snap);
    let _ = app.emit("snapshot", snap);
}

fn tick(app: &AppHandle) {
    let shared = app.state::<Shared>();
    let settings = shared.engine.lock().unwrap().settings.clone();
    // Sammeln ohne Lock – osascript/ps können dauern
    let obs = collect::observe(&settings);
    let (hint, snap, state) = {
        let mut e = shared.engine.lock().unwrap();
        let t = now();
        let hint = e.tick(t, &today(), obs);
        (hint, e.snapshot(t), e.state.clone())
    };
    save(&shared.dir.join("state.json"), &state);
    publish(app, &snap);
    if let Some(h) = hint {
        show_toast(app, &h);
    }
}

fn with_engine<R>(app: &AppHandle, f: impl FnOnce(&mut Engine, i64) -> R) -> R {
    let shared = app.state::<Shared>();
    let (r, snap, state) = {
        let mut e = shared.engine.lock().unwrap();
        let t = now();
        let r = f(&mut e, t);
        (r, e.snapshot(t), e.state.clone())
    };
    save(&shared.dir.join("state.json"), &state);
    publish(app, &snap);
    r
}

/// Aktionen aus Toast, Dashboard und Menü.
fn perform(app: &AppHandle, action: &str, kind: Option<HintKind>) {
    match action {
        "drink" => with_engine(app, |e, t| e.drink(t, None)),
        "posture_sitting" => with_engine(app, |e, t| e.set_posture(t, Posture::Sitting)),
        "posture_standing" => with_engine(app, |e, t| e.set_posture(t, Posture::Standing)),
        "posture_board" => with_engine(app, |e, t| e.set_posture(t, Posture::Board)),
        "break_start" => with_engine(app, |e, t| e.start_break(t)),
        "break_end" => with_engine(app, |e, t| e.finish_break(t)),
        "resume" => with_engine(app, |e, t| e.snooze(t, 0)),
        "dashboard" => show_dashboard(app),
        "quit" => app.exit(0),
        a if a.starts_with("snooze_") => {
            let min: u32 = a.trim_start_matches("snooze_").parse().unwrap_or(15);
            match kind {
                // „Gleich“ an einem Hinweis verschiebt nur diesen Hinweis
                Some(k) => with_engine(app, |e, t| e.dismiss(t, k, min)),
                None => with_engine(app, |e, t| e.snooze(t, min)),
            }
        }
        _ => {}
    }
}

// ---------- Commands ----------

#[tauri::command]
fn get_snapshot(app: AppHandle) -> Snapshot {
    let shared = app.state::<Shared>();
    let e = shared.engine.lock().unwrap();
    e.snapshot(now())
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Settings {
    app.state::<Shared>().engine.lock().unwrap().settings.clone()
}

#[tauri::command]
fn save_settings(app: AppHandle, settings: Settings) {
    let shared = app.state::<Shared>();
    save(&shared.dir.join("settings.json"), &settings);
    with_engine(&app, |e, _| e.settings = settings);
}

#[tauri::command]
fn act(app: AppHandle, action: String, kind: Option<HintKind>) {
    perform(&app, &action, kind);
}

#[tauri::command]
fn hide_toast(app: AppHandle) {
    if let Some(w) = app.get_webview_window("toast") {
        let _ = w.hide();
    }
}

#[tauri::command]
fn autostart_enabled(app: AppHandle) -> bool {
    app.autolaunch().is_enabled().unwrap_or(false)
}

#[tauri::command]
fn set_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    let al = app.autolaunch();
    if enabled { al.enable() } else { al.disable() }.map_err(|e| e.to_string())
}

#[tauri::command]
fn refresh(app: AppHandle) {
    std::thread::spawn(move || tick(&app));
}

// ---------- Start ----------

fn build_tray(app: &AppHandle) -> tauri::Result<MenuItem<tauri::Wry>> {
    let status = MenuItem::with_id(app, "status", "Lotse startet …", false, None::<&str>)?;
    let posture = Submenu::with_items(
        app,
        "Position",
        true,
        &[
            &MenuItem::with_id(app, "posture_sitting", "Swopper", true, None::<&str>)?,
            &MenuItem::with_id(app, "posture_standing", "Stehen", true, None::<&str>)?,
            &MenuItem::with_id(app, "posture_board", "Balance Board", true, None::<&str>)?,
        ],
    )?;
    let quiet = Submenu::with_items(
        app,
        "Hinweise pausieren",
        true,
        &[
            &MenuItem::with_id(app, "snooze_30", "30 Minuten", true, None::<&str>)?,
            &MenuItem::with_id(app, "snooze_60", "1 Stunde", true, None::<&str>)?,
            &MenuItem::with_id(app, "snooze_120", "2 Stunden", true, None::<&str>)?,
            &MenuItem::with_id(app, "resume", "Wieder aktivieren", true, None::<&str>)?,
        ],
    )?;
    let menu = Menu::with_items(
        app,
        &[
            &status,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "dashboard", "Cockpit öffnen", true, None::<&str>)?,
            &MenuItem::with_id(app, "drink", "Glas getrunken", true, None::<&str>)?,
            &posture,
            &MenuItem::with_id(app, "break_start", "Pause starten", true, None::<&str>)?,
            &MenuItem::with_id(app, "break_end", "Pause beenden", true, None::<&str>)?,
            &quiet,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "quit", "Lotse beenden", true, None::<&str>)?,
        ],
    )?;
    TrayIconBuilder::with_id("main")
        .icon(dot_icon(Level::Green, false))
        .icon_as_template(false)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| perform(app, event.id.as_ref(), None))
        .build(app)?;
    Ok(status)
}

pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .invoke_handler(tauri::generate_handler![
            get_snapshot,
            get_settings,
            save_settings,
            act,
            hide_toast,
            autostart_enabled,
            set_autostart,
            refresh
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let settings: Settings = load(&dir.join("settings.json"));
            let state: EngineState = load(&dir.join("state.json"));
            app.manage(Shared {
                engine: Mutex::new(Engine::new(settings, state)),
                dir,
                status_item: Mutex::new(None),
            });

            let status = build_tray(app.handle())?;
            *app.state::<Shared>().status_item.lock().unwrap() = Some(status);

            let handle = app.handle().clone();
            std::thread::spawn(move || loop {
                tick(&handle);
                std::thread::sleep(TICK);
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            // Cockpit schließen = verstecken; Lotse läuft in der Menüleiste weiter
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    let _ = window.hide();
                    api.prevent_close();
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("Lotse konnte nicht starten");

    app.run(|_app, event| {
        if let RunEvent::ExitRequested { api, code, .. } = event {
            if code.is_none() {
                api.prevent_exit();
            }
        }
    });
}
