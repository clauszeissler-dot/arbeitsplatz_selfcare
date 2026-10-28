//! Sammelt eine Beobachtung vom System. Läuft im Hintergrund-Thread.

use lotse_core::ghostty;
use lotse_core::observe::{self, SystemInput};
use lotse_core::procs::{self, ProcIndex};
use lotse_core::{Observation, Settings};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Führt ein Kommando mit Zeitlimit aus (osascript kann z. B. an einem
/// Berechtigungsdialog hängen).
fn run(cmd: &str, args: &[&str], timeout: Duration) -> Result<String, String> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if start.elapsed() > timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("Zeitüberschreitung".into());
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(40)),
            Err(e) => return Err(e.to_string()),
        }
    }
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

fn ghostty_note(err: &str) -> String {
    if err.contains("-1743") || err.contains("Not authorized") || err.contains("nicht berechtigt") {
        "Ghostty-Zugriff fehlt: Systemeinstellungen → Datenschutz & Sicherheit → Automation → Lotse → Ghostty aktivieren.".into()
    } else if err.contains("-2753") || err.contains("-1728") || err.contains("Can’t get") {
        "Ghostty ist zu alt für die Tab-Erkennung – bitte auf die neueste Version (≥ 1.3) aktualisieren.".into()
    } else {
        format!("Ghostty konnte nicht abgefragt werden: {err}")
    }
}

#[cfg(target_os = "macos")]
pub fn observe(settings: &Settings) -> Observation {
    let ps = run("/bin/ps", procs::PS_ARGS, Duration::from_secs(5)).unwrap_or_default();
    let index = ProcIndex::new(procs::parse_ps(&ps));

    let mut note = None;
    let ghostty = if index.is_running("/Ghostty.app/Contents/MacOS/") {
        match run("/usr/bin/osascript", &["-e", ghostty::SCRIPT], Duration::from_secs(8)) {
            Ok(out) => Some(ghostty::parse(&out)),
            Err(e) => {
                note = Some(ghostty_note(&e));
                None
            }
        }
    } else {
        None
    };

    let idle_secs = run("/usr/sbin/ioreg", lotse_core::idle::IOREG_ARGS, Duration::from_secs(5))
        .ok()
        .and_then(|o| lotse_core::idle::parse_ioreg(&o))
        .unwrap_or(0);

    let sys = SystemInput {
        idle_secs,
        mic_active: crate::macos_av::mic_in_use(),
        camera_active: crate::macos_av::camera_in_use(),
    };
    let home = std::env::var("HOME").unwrap_or_default();
    let mut obs = observe::build(ghostty.as_ref(), &index, sys, settings, &home);
    if note.is_some() {
        obs.ghostty_running = true;
        obs.ghostty_note = note;
    }
    obs
}

/// Windows/Linux: Platzhalter bis Phase 4 (Windows Terminal via UI Automation).
#[cfg(not(target_os = "macos"))]
pub fn observe(settings: &Settings) -> Observation {
    let ps = run("ps", &["-axo", "pid=,ppid=,pcpu=,tty=,args="], Duration::from_secs(5)).unwrap_or_default();
    let index = ProcIndex::new(procs::parse_ps(&ps));
    let sys = SystemInput { idle_secs: 0, mic_active: false, camera_active: false };
    let _ = ghostty_note;
    observe::build(None, &index, sys, settings, &std::env::var("HOME").unwrap_or_default())
}
