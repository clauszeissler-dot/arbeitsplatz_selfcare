mod collect;
mod exercise;
#[cfg(target_os = "macos")]
mod macos_av;
mod persistence;

use lotse_core::engine::State as EngineState;
use lotse_core::{Engine, Hint, HintKind, Level, Settings, Snapshot};
use persistence::{mutate_and_save, Persistence};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::Duration;
use tauri::image::Image;
use tauri::menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, LogicalPosition, Manager, RunEvent, WindowEvent};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

const TICK: Duration = Duration::from_secs(20);
const TOAST_W: f64 = 380.0;
const TOAST_MARGIN: f64 = 14.0;

struct Shared {
    engine: Mutex<Engine>,
    persistence: Persistence,
    storage_warning: Mutex<Option<String>>,
    exercise: Mutex<Option<exercise::Exercise>>,
    status_item: Mutex<Option<MenuItem<tauri::Wry>>>,
    ui_issues: Mutex<VecDeque<UiIssue>>,
}

fn now() -> i64 {
    chrono::Local::now().timestamp()
}

fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

#[derive(Clone, serde::Serialize)]
struct StorageHealth {
    ok: bool,
    message: Option<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct UiIssue {
    at: i64,
    context: String,
}

fn storage_health(shared: &Shared) -> StorageHealth {
    let message = shared.storage_warning.lock().unwrap().clone();
    StorageHealth {
        ok: message.is_none(),
        message,
    }
}

fn record_storage_result(app: &AppHandle, result: Result<(), String>) -> Result<(), String> {
    let shared = app.state::<Shared>();
    const STORAGE_ERROR: &str = "Lokale Daten konnten nicht sicher gespeichert werden.";
    let mut warning = shared.storage_warning.lock().unwrap();
    match &result {
        Err(_) => *warning = Some(STORAGE_ERROR.to_string()),
        Ok(()) if warning.as_deref() == Some(STORAGE_ERROR) => *warning = None,
        Ok(()) => {}
    }
    drop(warning);
    let _ = app.emit("storage_health", storage_health(&shared));
    result
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
    let line = if shared.storage_warning.lock().unwrap().is_some() {
        "Speicherfehler · Cockpit öffnen".to_string()
    } else {
        status_line(snap)
    };
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_icon(Some(dot_icon(snap.level, snap.in_meeting)));
        let title = if snap.in_break {
            "Pause".to_string()
        } else {
            format!("{}′", snap.block_min)
        };
        let _ = tray.set_title(Some(title));
        let _ = tray.set_tooltip(Some(line.clone()));
    }
    let item = shared.status_item.lock().unwrap().clone();
    if let Some(item) = item {
        let _ = item.set_text(&line);
    }
}

// ---------- Fenster ----------

fn show_dashboard(app: &AppHandle) {
    #[cfg(target_os = "macos")]
    let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

fn show_toast(app: &AppHandle, hint: &Hint) -> bool {
    let shared = app.state::<Shared>();
    if shared
        .exercise
        .lock()
        .unwrap()
        .as_ref()
        .is_some_and(|e| e.deadline.is_some())
    {
        return false;
    }
    let Some(w) = app.get_webview_window("toast") else {
        return false;
    };
    if let Ok(Some(m)) = w.primary_monitor() {
        let scale = m.scale_factor();
        let size = m.size().to_logical::<f64>(scale);
        let pos = m.position().to_logical::<f64>(scale);
        let _ = w.set_position(LogicalPosition::new(
            pos.x + size.width - TOAST_W - TOAST_MARGIN,
            pos.y + 38.0,
        ));
    }
    // Monitor queries above can wait on the UI thread; never hold the exercise lock there.
    let mut exercise = shared.exercise.lock().unwrap();
    if exercise.as_ref().is_some_and(|e| e.deadline.is_some()) {
        return false;
    }
    *exercise = Some(exercise::Exercise {
        hint: hint.clone(),
        deadline: None,
        finished: false,
        sound: true,
    });
    if w.show().is_err() {
        *exercise = None;
        return false;
    }
    let _ = app.emit_to("toast", "hint", hint);
    true
}

/// Originale lokale WAV-Dateien; der Playback-Prozess wird außerhalb des UI-Threads abgewartet.
#[tauri::command]
fn play_sound(app: AppHandle, name: String) -> Result<(), String> {
    let bytes: &[u8] = match name.as_str() {
        "hint" => include_bytes!("../../ui/src/audio/hint.wav"),
        "start" => include_bytes!("../../ui/src/audio/start.wav"),
        "complete" => include_bytes!("../../ui/src/audio/complete.wav"),
        _ => return Err("Unbekannter Klang".into()),
    };
    let path =
        app.state::<Shared>()
            .persistence
            .write_private("sounds", &format!("{name}.wav"), bytes)?;
    #[cfg(target_os = "macos")]
    std::thread::spawn(move || {
        let _ = std::process::Command::new("/usr/bin/afplay")
            .arg(path)
            .status();
    });
    Ok(())
}

#[derive(Clone, serde::Serialize)]
struct TimerUpdate {
    remaining: u32,
    finished: bool,
}

#[tauri::command]
fn start_hint_timer(app: AppHandle, sound: bool) -> Result<(), String> {
    let kind = {
        let shared = app.state::<Shared>();
        let mut guard = shared.exercise.lock().unwrap();
        let exercise = guard.as_mut().ok_or("Kein aktueller Hinweis")?;
        exercise.start(std::time::SystemTime::now(), sound)?;
        exercise.hint.kind
    };
    if matches!(kind, HintKind::Break | HintKind::LongBreak) {
        if let Err(error) = perform(&app, "break_start", Some(kind)) {
            if let Some(exercise) = app.state::<Shared>().exercise.lock().unwrap().as_mut() {
                exercise.deadline = None;
                exercise.finished = false;
            }
            return Err(error);
        }
    }
    if sound {
        let _ = play_sound(app, "start".into());
    }
    Ok(())
}

fn timer_tick(app: &AppHandle) {
    let shared = app.state::<Shared>();
    let mut guard = shared.exercise.lock().unwrap();
    if let Some(exercise) = guard
        .as_mut()
        .filter(|e| e.deadline.is_some() && !e.finished)
    {
        let now = std::time::SystemTime::now();
        let finished = exercise.finish_if_due(now);
        let _ = app.emit_to(
            "toast",
            "timer_update",
            TimerUpdate {
                remaining: exercise.remaining(now),
                finished,
            },
        );
        if finished && exercise.sound {
            let _ = play_sound(app.clone(), "complete".into());
        }
    }
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
    let persisted = mutate_and_save(
        &shared.engine,
        &shared.persistence,
        "state.json",
        |e| {
            let t = now();
            let hint = e.tick(t, &today(), obs);
            (hint, e.snapshot(t))
        },
        |e| &e.state,
    );
    let (hint, snap) = persisted.value;
    let _ = record_storage_result(app, persisted.result);
    publish(app, &snap);
    if let Some(h) = hint {
        if show_toast(app, &h) && settings.sound_enabled {
            let _ = play_sound(app.clone(), "hint".into());
        }
    }
}

fn with_engine<R>(app: &AppHandle, f: impl FnOnce(&mut Engine, i64) -> R) -> Result<R, String> {
    let shared = app.state::<Shared>();
    let persisted = mutate_and_save(
        &shared.engine,
        &shared.persistence,
        "state.json",
        |e| {
            let t = now();
            let result = f(e, t);
            (result, e.snapshot(t))
        },
        |e| &e.state,
    );
    let (result, snap) = persisted.value;
    let saved = record_storage_result(app, persisted.result);
    publish(app, &snap);
    saved.map(|()| result)
}

/// Aktionen aus Toast, Dashboard und Menü.
fn perform(app: &AppHandle, action: &str, kind: Option<HintKind>) -> Result<(), String> {
    match action {
        "drink" => with_engine(app, |e, t| e.drink(t, None)).map(|_| ()),
        a if a.starts_with("posture_") => {
            let id = a.trim_start_matches("posture_");
            with_engine(app, |e, t| e.set_posture(t, id)).map(|_| ())
        }
        "break_start" => with_engine(app, |e, t| e.start_break(t)).map(|_| ()),
        "break_end" => with_engine(app, |e, t| e.finish_break(t)).map(|_| ()),
        "resume" => with_engine(app, |e, t| e.snooze(t, 0)).map(|_| ()),
        "dashboard" => {
            show_dashboard(app);
            Ok(())
        }
        "quit" => {
            app.exit(0);
            Ok(())
        }
        a if a.starts_with("snooze_") => {
            let min: u32 = a.trim_start_matches("snooze_").parse().unwrap_or(15);
            match kind {
                // „Gleich“ an einem Hinweis verschiebt nur diesen Hinweis
                Some(k) => with_engine(app, |e, t| e.dismiss(t, k, min)).map(|_| ()),
                None => with_engine(app, |e, t| e.snooze(t, min)).map(|_| ()),
            }
        }
        _ => Ok(()),
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
    app.state::<Shared>()
        .engine
        .lock()
        .unwrap()
        .settings
        .clone()
}

#[tauri::command]
fn get_storage_health(app: AppHandle) -> StorageHealth {
    storage_health(&app.state::<Shared>())
}

#[tauri::command]
fn get_current_hint(app: AppHandle) -> Option<Hint> {
    app.state::<Shared>()
        .exercise
        .lock()
        .unwrap()
        .as_ref()
        .map(|exercise| exercise.hint.clone())
}

#[tauri::command]
fn report_ui_issue(app: AppHandle, context: String) -> Result<(), String> {
    const ALLOWED: &[&str] = &[
        "dashboard-load",
        "dashboard-listener",
        "dashboard-runtime",
        "toast-load",
        "toast-listener",
        "toast-runtime",
    ];
    if !ALLOWED.contains(&context.as_str()) {
        return Err("Unbekannter Diagnosekontext".into());
    }
    let shared = app.state::<Shared>();
    let mut issues = shared.ui_issues.lock().unwrap();
    issues.push_back(UiIssue { at: now(), context });
    while issues.len() > 50 {
        issues.pop_front();
    }
    shared.persistence.save("ui-diagnostics.json", &*issues)
}

#[tauri::command]
fn save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    let shared = app.state::<Shared>();
    let mut settings = settings;
    settings.normalize_postures();
    let (snap, result) = {
        let mut engine = shared.engine.lock().unwrap();
        match shared.persistence.save("settings.json", &settings) {
            Ok(()) => {
                engine.update_settings(settings);
                let snap = engine.snapshot(now());
                (
                    Some(snap),
                    shared.persistence.save("state.json", &engine.state),
                )
            }
            Err(error) => (None, Err(error)),
        }
    };
    let saved = record_storage_result(&app, result);
    if let Some(snap) = snap {
        publish(&app, &snap);
    }
    saved
}

#[tauri::command]
fn act(app: AppHandle, action: String, kind: Option<HintKind>) -> Result<(), String> {
    perform(&app, &action, kind)
}

#[tauri::command]
fn hide_toast(app: AppHandle) {
    let ended = exercise::dismiss(&mut app.state::<Shared>().exercise.lock().unwrap());
    if ended.is_some_and(|e| {
        e.deadline.is_some() && matches!(e.hint.kind, HintKind::Break | HintKind::LongBreak)
    }) {
        let _ = perform(&app, "break_end", None);
    }
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
    let postures = app
        .state::<Shared>()
        .engine
        .lock()
        .unwrap()
        .settings
        .postures
        .clone();
    let posture_items = postures
        .iter()
        .map(|posture| {
            MenuItem::with_id(
                app,
                format!("posture_{}", posture.id),
                &posture.label,
                true,
                None::<&str>,
            )
        })
        .collect::<tauri::Result<Vec<_>>>()?;
    let posture_refs: Vec<&dyn IsMenuItem<tauri::Wry>> = posture_items
        .iter()
        .map(|item| item as &dyn IsMenuItem<tauri::Wry>)
        .collect();
    let posture = Submenu::with_items(app, "Position", true, &posture_refs)?;
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
        .on_menu_event(|app, event| {
            let _ = perform(app, event.id.as_ref(), None);
        })
        .build(app)?;
    Ok(status)
}

pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            get_snapshot,
            get_settings,
            get_storage_health,
            get_current_hint,
            report_ui_issue,
            save_settings,
            act,
            hide_toast,
            autostart_enabled,
            set_autostart,
            refresh,
            play_sound,
            start_hint_timer
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let persistence =
                Persistence::open(app.path().app_data_dir()?).map_err(std::io::Error::other)?;
            let settings = persistence
                .load::<Settings>("settings.json")
                .map_err(std::io::Error::other)?;
            let state = persistence
                .load::<EngineState>("state.json")
                .map_err(std::io::Error::other)?;
            let ui_issues = persistence
                .load::<VecDeque<UiIssue>>("ui-diagnostics.json")
                .map_err(std::io::Error::other)?;
            let load_warning = settings
                .warning
                .or(state.warning)
                .or(ui_issues.warning)
                .map(|_| {
                    "Eine beschädigte lokale Datendatei wurde gesichert und neu angelegt."
                        .to_string()
                });
            app.manage(Shared {
                engine: Mutex::new(Engine::new(settings.value, state.value)),
                persistence,
                storage_warning: Mutex::new(load_warning),
                exercise: Mutex::new(None),
                status_item: Mutex::new(None),
                ui_issues: Mutex::new(ui_issues.value),
            });

            let status = build_tray(app.handle())?;
            *app.state::<Shared>().status_item.lock().unwrap() = Some(status);

            let timer_handle = app.handle().clone();
            std::thread::spawn(move || loop {
                timer_tick(&timer_handle);
                std::thread::sleep(Duration::from_millis(200));
            });
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
                    #[cfg(target_os = "macos")]
                    let _ = window
                        .app_handle()
                        .set_activation_policy(tauri::ActivationPolicy::Accessory);
                    api.prevent_close();
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("Lotse konnte nicht starten");

    app.run(|app, event| {
        #[cfg(target_os = "macos")]
        if let RunEvent::Reopen { .. } = event {
            show_dashboard(app);
        }
        if let RunEvent::ExitRequested { api, code, .. } = event {
            if code.is_none() {
                api.prevent_exit();
            }
        }
    });
}
