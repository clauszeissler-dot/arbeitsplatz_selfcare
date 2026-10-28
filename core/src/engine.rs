//! Die Erinnerungs- und Ampel-Engine.
//!
//! Rein funktional gegenüber der Außenwelt: bekommt pro Takt eine
//! [`Observation`] plus Uhrzeit und liefert höchstens einen [`Hint`].
//! Der Zustand ist serialisierbar und überlebt Neustarts.

use crate::observe::Observation;
use crate::settings::{Band, Settings};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    #[default]
    Green,
    Yellow,
    Red,
}

impl Level {
    pub fn of(value: f64, band: Band) -> Self {
        if value <= band.green {
            Level::Green
        } else if value <= band.yellow {
            Level::Yellow
        } else {
            Level::Red
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "lowercase")]
pub enum Posture {
    #[default]
    Sitting,
    Standing,
    Board,
}

impl Posture {
    pub fn label(self) -> &'static str {
        match self {
            Posture::Sitting => "Swopper",
            Posture::Standing => "Stehen",
            Posture::Board => "Balance Board",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HintKind {
    LongBreak,
    Break,
    Load,
    Posture,
    Water,
    Eyes,
    DayEnd,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HintAction {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hint {
    pub kind: HintKind,
    pub level: Level,
    pub title: String,
    pub body: String,
    pub actions: Vec<HintAction>,
    pub at: i64,
}

fn action(id: &str, label: &str) -> HintAction {
    HintAction {
        id: id.into(),
        label: label.into(),
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BreakRecord {
    pub start: i64,
    pub end: i64,
}

impl BreakRecord {
    pub fn minutes(&self) -> i64 {
        (self.end - self.start) / 60
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DaySummary {
    pub day: String,
    pub active_min: u64,
    pub water_ml: u32,
    pub breaks: u32,
    pub long_breaks: u32,
    pub posture_min: BTreeMap<Posture, u64>,
    pub red_min: u64,
    pub max_projects: u32,
    pub max_busy_agents: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct State {
    pub day: String,
    pub first_active: Option<i64>,
    pub active_secs: u64,
    pub red_secs: u64,
    pub water_ml: u32,
    pub last_drink: Option<i64>,
    pub posture: Posture,
    pub posture_since: i64,
    pub posture_secs: BTreeMap<Posture, u64>,
    pub block_start: i64,
    pub last_long_break_end: i64,
    pub breaks: Vec<BreakRecord>,
    pub idle_started: Option<i64>,
    pub manual_break: Option<i64>,
    pub last_tick: Option<i64>,
    pub next_allowed: BTreeMap<HintKind, i64>,
    pub last_hint_at: i64,
    pub snooze_until: Option<i64>,
    pub mic_since: Option<i64>,
    pub meeting_ended: Option<i64>,
    pub in_meeting: bool,
    pub switches: VecDeque<i64>,
    pub last_focus: Option<String>,
    pub red_since: Option<i64>,
    pub max_projects: u32,
    pub max_busy_agents: u32,
    pub history: Vec<DaySummary>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            day: String::new(),
            first_active: None,
            active_secs: 0,
            red_secs: 0,
            water_ml: 0,
            last_drink: None,
            posture: Posture::Sitting,
            posture_since: 0,
            posture_secs: BTreeMap::new(),
            block_start: 0,
            last_long_break_end: 0,
            breaks: Vec::new(),
            idle_started: None,
            manual_break: None,
            last_tick: None,
            next_allowed: BTreeMap::new(),
            last_hint_at: 0,
            snooze_until: None,
            mic_since: None,
            meeting_ended: None,
            in_meeting: false,
            switches: VecDeque::new(),
            last_focus: None,
            red_since: None,
            max_projects: 0,
            max_busy_agents: 0,
            history: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    pub key: String,
    pub label: String,
    pub value: f64,
    pub display: String,
    pub level: Level,
    pub hint: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub at: i64,
    pub level: Level,
    pub metrics: Vec<Metric>,
    pub reasons: Vec<String>,
    pub in_meeting: bool,
    pub in_break: bool,
    pub snoozed_until: Option<i64>,
    pub active_min: u64,
    pub workday_min: u32,
    pub block_min: i64,
    pub since_long_break_min: i64,
    pub water_ml: u32,
    pub water_target_ml: u32,
    pub water_expected_ml: u32,
    pub glass_ml: u32,
    pub posture: Posture,
    pub posture_label: String,
    pub posture_min: i64,
    pub posture_limit_min: u32,
    pub posture_share: BTreeMap<Posture, u64>,
    pub breaks_today: u32,
    pub long_breaks_today: u32,
    pub next_water_min: Option<i64>,
    pub next_break_min: i64,
    pub observation: Observation,
    pub history: Vec<DaySummary>,
}

pub struct Engine {
    pub settings: Settings,
    pub state: State,
    pub last_obs: Observation,
}

const MAX_TICK_GAP: i64 = 90;
const SLEEP_GAP: i64 = 600;
const LOAD_RED_HOLD: i64 = 300;

impl Engine {
    pub fn new(settings: Settings, state: State) -> Self {
        Self {
            settings,
            state,
            last_obs: Observation::default(),
        }
    }

    fn mins(&self, m: u32) -> i64 {
        m as i64 * 60
    }

    fn start_day(&mut self, day: &str, now: i64) {
        if !self.state.day.is_empty() {
            let s = &self.state;
            let summary = DaySummary {
                day: s.day.clone(),
                active_min: s.active_secs / 60,
                water_ml: s.water_ml,
                breaks: s.breaks.len() as u32,
                long_breaks: s
                    .breaks
                    .iter()
                    .filter(|b| b.minutes() >= self.settings.long_break_min as i64)
                    .count() as u32,
                posture_min: s.posture_secs.iter().map(|(k, v)| (*k, v / 60)).collect(),
                red_min: s.red_secs / 60,
                max_projects: s.max_projects,
                max_busy_agents: s.max_busy_agents,
            };
            let mut history = std::mem::take(&mut self.state.history);
            history.push(summary);
            if history.len() > 60 {
                history.remove(0);
            }
            self.state = State {
                history,
                ..State::default()
            };
        }
        self.state.day = day.to_string();
        self.state.block_start = now;
        self.state.last_long_break_end = now;
        self.state.posture_since = now;
    }

    fn end_break(&mut self, start: i64, end: i64) {
        let len = end - start;
        if len < self.mins(self.settings.break_min) {
            return;
        }
        self.state.breaks.push(BreakRecord { start, end });
        self.state.block_start = end;
        if len >= self.mins(self.settings.long_break_min) {
            self.state.last_long_break_end = end;
        }
        // Wer Pause gemacht hat, hat sich bewegt – Haltungsuhr neu starten
        self.state.posture_since = end;
    }

    // ---------- Aktionen aus der Oberfläche ----------

    pub fn drink(&mut self, now: i64, ml: Option<u32>) {
        let ml = ml.unwrap_or(self.settings.glass_ml);
        self.state.water_ml += ml;
        self.state.last_drink = Some(now);
        self.state.next_allowed.insert(
            HintKind::Water,
            now + self.mins(self.settings.water_interval_min),
        );
    }

    pub fn set_posture(&mut self, now: i64, p: Posture) {
        if self.state.posture != p {
            self.state.posture = p;
        }
        self.state.posture_since = now;
        self.state.next_allowed.remove(&HintKind::Posture);
    }

    pub fn start_break(&mut self, now: i64) {
        self.state.manual_break = Some(now);
    }

    pub fn finish_break(&mut self, now: i64) {
        if let Some(start) = self.state.manual_break.take() {
            // Manuelle Pause zählt mindestens als kurze Pause
            let start = start.min(now - self.mins(self.settings.break_min));
            self.end_break(start, now);
        }
    }

    pub fn snooze(&mut self, now: i64, minutes: u32) {
        self.state.snooze_until = if minutes == 0 {
            None
        } else {
            Some(now + minutes as i64 * 60)
        };
    }

    pub fn dismiss(&mut self, now: i64, kind: HintKind, minutes: u32) {
        self.state
            .next_allowed
            .insert(kind, now + minutes as i64 * 60);
    }

    // ---------- Takt ----------

    /// Verarbeitet einen Takt. `day` ist das lokale Datum (YYYY-MM-DD).
    pub fn tick(&mut self, now: i64, day: &str, obs: Observation) -> Option<Hint> {
        if self.state.day != day {
            self.start_day(day, now);
        }
        let s = self.settings.clone();
        let gap = self.state.last_tick.map(|t| now - t).unwrap_or(0);

        // Rechner geschlafen / Deckel zu ⇒ das war eine Pause
        if gap > SLEEP_GAP {
            let start = self.state.last_tick.unwrap_or(now);
            self.end_break(start, now);
            self.state.idle_started = None;
        }
        let dt = gap.clamp(0, MAX_TICK_GAP) as u64;

        // Meeting: Kamera sofort, Mikrofon erst nach einer Weile (Diktat ≠ Meeting)
        if obs.mic_active && s.meeting_use_mic {
            self.state.mic_since.get_or_insert(now);
        } else {
            self.state.mic_since = None;
        }
        let mic_meeting = self
            .state
            .mic_since
            .map(|t| now - t >= s.meeting_mic_secs as i64)
            .unwrap_or(false);
        let in_meeting = obs.camera_active || mic_meeting;
        if self.state.in_meeting && !in_meeting {
            self.state.meeting_ended = Some(now);
        }
        self.state.in_meeting = in_meeting;

        // Pausen anhand der Leerlaufzeit (nicht im Meeting – da tippt man nicht)
        let idle = obs.idle_secs as i64;
        let active = idle < s.idle_active_secs as i64 || in_meeting;
        if in_meeting || self.state.manual_break.is_some() {
            self.state.idle_started = None;
        } else if idle >= s.idle_active_secs as i64 {
            self.state.idle_started.get_or_insert(now - idle);
        } else if let Some(start) = self.state.idle_started.take() {
            self.end_break(start, now - idle);
        }
        if self.state.manual_break.is_some() && idle < 5 && gap > 0 {
            // Wer nach manueller Pause wieder tippt, ist zurück, sobald die Mindestdauer um ist
            let start = self.state.manual_break.unwrap_or(now);
            if now - start >= self.mins(s.break_min) {
                self.finish_break(now);
            }
        }

        if active && self.state.manual_break.is_none() {
            self.state.active_secs += dt;
            self.state.first_active.get_or_insert(now);
            *self
                .state
                .posture_secs
                .entry(self.state.posture)
                .or_default() += dt;
        }

        // Tab-Wechsel zählen
        if let Some(focus) = &obs.focused_key {
            if self.state.last_focus.as_ref() != Some(focus) {
                if self.state.last_focus.is_some() {
                    self.state.switches.push_back(now);
                }
                self.state.last_focus = Some(focus.clone());
            }
        }
        while self
            .state
            .switches
            .front()
            .map(|t| now - t > 600)
            .unwrap_or(false)
        {
            self.state.switches.pop_front();
        }

        self.state.max_projects = self.state.max_projects.max(obs.projects().len() as u32);
        self.state.max_busy_agents = self.state.max_busy_agents.max(obs.busy_agents() as u32);
        self.last_obs = obs;
        self.state.last_tick = Some(now);

        let (level, _, reasons) = self.assess(now);
        if level == Level::Red {
            self.state.red_secs += dt;
            self.state.red_since.get_or_insert(now);
        } else {
            self.state.red_since = None;
        }

        if !active || self.state.manual_break.is_some() {
            return None;
        }
        self.pick_hint(now, level, &reasons)
    }

    fn in_break(&self, now: i64) -> bool {
        self.state.manual_break.is_some()
            || self
                .state
                .idle_started
                .map(|t| now - t >= self.mins(self.settings.break_min))
                .unwrap_or(false)
    }

    fn block_secs(&self, now: i64) -> i64 {
        if self.in_break(now) {
            0
        } else {
            now - self.state.block_start
        }
    }

    fn posture_limit(&self) -> u32 {
        match self.state.posture {
            Posture::Sitting => self.settings.sit_minutes,
            Posture::Standing => self.settings.stand_minutes,
            Posture::Board => self.settings.board_minutes,
        }
    }

    fn water_expected(&self, now: i64) -> u32 {
        let s = &self.settings;
        let Some(first) = self.state.first_active else {
            return 0;
        };
        let elapsed = (now - first).max(0) as f64 / 60.0;
        let share = (elapsed / s.workday_minutes.max(1) as f64).min(1.0);
        (s.water_target_ml as f64 * share) as u32
    }

    fn water_due_in(&self, now: i64) -> Option<i64> {
        let s = &self.settings;
        if self.state.water_ml >= s.water_target_ml {
            return None;
        }
        let behind = self.water_expected(now).saturating_sub(self.state.water_ml) >= 2 * s.glass_ml;
        let interval = if behind {
            self.mins(s.water_interval_min) / 2
        } else {
            self.mins(s.water_interval_min)
        };
        let last = self
            .state
            .last_drink
            .or(self.state.first_active)
            .unwrap_or(now);
        Some(last + interval - now)
    }

    /// Ampel: jede Kennzahl einzeln, gesamt = schlechteste.
    pub fn assess(&self, now: i64) -> (Level, Vec<Metric>, Vec<String>) {
        let t = &self.settings.thresholds;
        let o = &self.last_obs;
        let projects = o.projects().len() as f64;
        let tabs = o.tabs.len() as f64;
        let busy = o.busy_agents() as f64;
        let block = (self.block_secs(now) / 60) as f64;
        let switches = self.state.switches.len() as f64;
        let hours = self.state.active_secs as f64 / 3600.0;

        let mut metrics = vec![
            Metric {
                key: "projects".into(),
                label: "Parallele Projekte".into(),
                value: projects,
                display: format!("{projects}"),
                level: Level::of(projects, t.projects),
                hint: "Optimum liegt bei 2 – ab 3 sinkt die wertschöpfende Zeit je Projekt deutlich.".into(),
            },
            Metric {
                key: "agents".into(),
                label: "KI arbeitet gerade".into(),
                value: busy,
                display: format!("{busy} / {}", o.total_agents() + o.apps.len()),
                level: Level::of(busy, t.busy_agents),
                hint: "Mehr als 2 gleichzeitig zu überwachende KI-Stränge erhöhen die mentale Ermüdung.".into(),
            },
            Metric {
                key: "block".into(),
                label: "Seit letzter Pause".into(),
                value: block,
                display: format!("{block} min"),
                level: Level::of(block, t.block_minutes),
                hint: "Fluglotsen: höchstens 2 h am Stück, dann 30 min ohne Verantwortung.".into(),
            },
            Metric {
                key: "tabs".into(),
                label: "Offene Tabs".into(),
                value: tabs,
                display: format!("{tabs}"),
                level: Level::of(tabs, t.tabs),
                hint: "Jeder offene Tab ist ein offener Gedanke. Parken statt offen halten.".into(),
            },
            Metric {
                key: "switches".into(),
                label: "Tab-Wechsel / 10 min".into(),
                value: switches,
                display: format!("{switches}"),
                level: Level::of(switches, t.switches_10min),
                hint: "Jeder Wechsel hinterlässt Aufmerksamkeitsreste beim vorherigen Thema.".into(),
            },
            Metric {
                key: "day".into(),
                label: "Aktiv heute".into(),
                value: hours,
                display: format!("{}:{:02} h", self.state.active_secs / 3600, (self.state.active_secs % 3600) / 60),
                level: Level::of(hours, t.day_hours),
                hint: "Kognitive Leistung sinkt am Ende langer Tage messbar.".into(),
            },
        ];
        if !o.ghostty_running {
            metrics.retain(|m| m.key != "tabs" && m.key != "switches");
        }
        let level = metrics.iter().map(|m| m.level).max().unwrap_or_default();
        let reasons = metrics
            .iter()
            .filter(|m| m.level == level && level != Level::Green)
            .map(|m| format!("{}: {}", m.label, m.display))
            .collect();
        (level, metrics, reasons)
    }

    fn allowed(&self, now: i64, kind: HintKind) -> bool {
        self.state
            .next_allowed
            .get(&kind)
            .map(|t| now >= *t)
            .unwrap_or(true)
    }

    fn pick_hint(&mut self, now: i64, level: Level, reasons: &[String]) -> Option<Hint> {
        let s = self.settings.clone();
        if self.state.in_meeting {
            return None;
        }
        if self
            .state
            .meeting_ended
            .map(|t| now - t < 60)
            .unwrap_or(false)
        {
            return None;
        }
        if self.state.snooze_until.map(|t| now < t).unwrap_or(false) {
            return None;
        }
        if now - self.state.last_hint_at < s.min_gap_secs as i64 {
            return None;
        }

        let block_min = self.block_secs(now) / 60;
        let since_long = (now - self.state.last_long_break_end) / 60;
        let mut candidates: Vec<(HintKind, i64, Hint)> = Vec::new();
        let mk = |kind, level, title: String, body: String, actions: Vec<HintAction>| Hint {
            kind,
            level,
            title,
            body,
            actions,
            at: now,
        };

        if since_long >= s.long_break_after_min as i64 {
            candidates.push((
                HintKind::LongBreak,
                15,
                mk(
                    HintKind::LongBreak,
                    Level::Red,
                    format!("{} h ohne echte Pause", since_long / 60),
                    "Lotsen-Regel: 30 Minuten raus, ohne Verantwortung. Agenten laufen lassen oder parken, Bildschirm aus.".into(),
                    vec![action("break_start", "Pause starten"), action("snooze_15", "In 15 min")],
                ),
            ));
        }
        if block_min >= s.block_hard_min as i64 {
            candidates.push((
                HintKind::Break,
                10,
                mk(
                    HintKind::Break,
                    Level::Red,
                    format!("{block_min} min am Stück"),
                    "Zeit für 10–15 Minuten weg vom Bildschirm. Vorher kurz notieren, wo du stehst.".into(),
                    vec![action("break_start", "Pause starten"), action("snooze_10", "In 10 min")],
                ),
            ));
        } else if block_min >= s.block_soft_min as i64 {
            candidates.push((
                HintKind::Break,
                25,
                mk(
                    HintKind::Break,
                    Level::Yellow,
                    "Pause einplanen".into(),
                    format!("{block_min} min Fokus. Bring den aktuellen Strang zu Ende und plane die nächste Pause."),
                    vec![action("break_start", "Jetzt Pause"), action("ok", "Gleich")],
                ),
            ));
        }
        if level == Level::Red
            && self
                .state
                .red_since
                .map(|t| now - t >= LOAD_RED_HOLD)
                .unwrap_or(false)
        {
            candidates.push((
                HintKind::Load,
                30,
                mk(
                    HintKind::Load,
                    Level::Red,
                    "Zu viel parallel".into(),
                    format!(
                        "{}. Park einen Strang: Notiz schreiben, Tab schließen.",
                        reasons.join(" · ")
                    ),
                    vec![action("ok", "Verstanden")],
                ),
            ));
        }
        let posture_min = (now - self.state.posture_since) / 60;
        if posture_min >= self.posture_limit() as i64 {
            let (title, body, acts) = match self.state.posture {
                Posture::Sitting => (
                    "Aufstehen".to_string(),
                    format!("{posture_min} min gesessen. Jetzt {} min stehen – gern ein paar davon aufs Board.", s.stand_minutes),
                    vec![action("posture_standing", "Stehe"), action("posture_board", "Board")],
                ),
                Posture::Standing => (
                    "Haltung wechseln".to_string(),
                    format!("{posture_min} min gestanden. Zurück auf den Swopper oder 5 min aufs Board."),
                    vec![action("posture_sitting", "Swopper"), action("posture_board", "Board")],
                ),
                Posture::Board => (
                    "Runter vom Board".to_string(),
                    format!("{posture_min} min Balance. Weiter stehen oder auf den Swopper."),
                    vec![action("posture_standing", "Stehen"), action("posture_sitting", "Swopper")],
                ),
            };
            candidates.push((
                HintKind::Posture,
                10,
                mk(HintKind::Posture, Level::Yellow, title, body, acts),
            ));
        }
        if let Some(due) = self.water_due_in(now) {
            if due <= 0 {
                let missing = s.water_target_ml.saturating_sub(self.state.water_ml);
                candidates.push((
                    HintKind::Water,
                    15,
                    mk(
                        HintKind::Water,
                        Level::Yellow,
                        "Trinken".into(),
                        format!(
                            "{:.1} von {:.1} l. Ein Glas ({} ml) jetzt – noch {:.1} l bis zum Ziel.",
                            self.state.water_ml as f64 / 1000.0,
                            s.water_target_ml as f64 / 1000.0,
                            s.glass_ml,
                            missing as f64 / 1000.0
                        ),
                        vec![action("drink", "Getrunken"), action("snooze_10", "Gleich")],
                    ),
                ));
            }
        }
        if s.eyes_enabled && block_min > 0 && block_min % s.eyes_interval_min.max(1) as i64 == 0 {
            candidates.push((
                HintKind::Eyes,
                s.eyes_interval_min as i64,
                mk(
                    HintKind::Eyes,
                    Level::Green,
                    "Augen kurz raus".into(),
                    "20 Sekunden auf etwas in mindestens 6 m Entfernung schauen.".into(),
                    vec![action("ok", "Erledigt")],
                ),
            ));
        }
        if self.state.active_secs >= s.workday_minutes as u64 * 60 {
            candidates.push((
                HintKind::DayEnd,
                30,
                mk(
                    HintKind::DayEnd,
                    Level::Red,
                    "Arbeitstag ist voll".into(),
                    format!(
                        "{} h aktiv. Offene Stränge notieren, Agenten parken, Feierabend.",
                        self.state.active_secs / 3600
                    ),
                    vec![
                        action("ok", "Ich mache Schluss"),
                        action("snooze_30", "Noch 30 min"),
                    ],
                ),
            ));
        }

        candidates.sort_by_key(|(k, _, _)| *k);
        let (kind, cooldown, hint) = candidates
            .into_iter()
            .find(|(k, _, _)| self.allowed(now, *k))?;
        self.state.next_allowed.insert(kind, now + cooldown * 60);
        self.state.last_hint_at = now;
        Some(hint)
    }

    pub fn snapshot(&self, now: i64) -> Snapshot {
        let (level, metrics, reasons) = self.assess(now);
        let s = &self.settings;
        let next_break = (s.block_soft_min as i64 * 60 - self.block_secs(now)).max(0) / 60;
        Snapshot {
            at: now,
            level,
            metrics,
            reasons,
            in_meeting: self.state.in_meeting,
            in_break: self.in_break(now),
            snoozed_until: self.state.snooze_until.filter(|t| *t > now),
            active_min: self.state.active_secs / 60,
            workday_min: s.workday_minutes,
            block_min: self.block_secs(now) / 60,
            since_long_break_min: (now - self.state.last_long_break_end) / 60,
            water_ml: self.state.water_ml,
            water_target_ml: s.water_target_ml,
            water_expected_ml: self.water_expected(now),
            glass_ml: s.glass_ml,
            posture: self.state.posture,
            posture_label: self.state.posture.label().into(),
            posture_min: (now - self.state.posture_since) / 60,
            posture_limit_min: self.posture_limit(),
            posture_share: self
                .state
                .posture_secs
                .iter()
                .map(|(k, v)| (*k, v / 60))
                .collect(),
            breaks_today: self.state.breaks.len() as u32,
            long_breaks_today: self
                .state
                .breaks
                .iter()
                .filter(|b| b.minutes() >= s.long_break_min as i64)
                .count() as u32,
            next_water_min: self.water_due_in(now).map(|d| d.max(0) / 60),
            next_break_min: next_break,
            observation: self.last_obs.clone(),
            history: self.state.history.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observe::{AgentInfo, TabView};

    const DAY: &str = "2026-10-23";
    const T0: i64 = 1_800_000_000;

    fn obs(idle: u64) -> Observation {
        Observation {
            ghostty_running: true,
            idle_secs: idle,
            ..Default::default()
        }
    }

    fn tab(key: &str, project: &str, busy: bool) -> TabView {
        TabView {
            key: key.into(),
            title: key.into(),
            project: Some(project.into()),
            cwd: project.into(),
            focused: false,
            agents: vec![AgentInfo {
                name: "Claude Code".into(),
                busy,
                cpu: 10.0,
            }],
        }
    }

    /// Lässt `minutes` Minuten aktiver Arbeit im 30-s-Takt laufen und sammelt Hinweise.
    fn run(e: &mut Engine, from: i64, minutes: i64, o: &Observation) -> Vec<Hint> {
        let mut out = Vec::new();
        let mut t = from;
        while t < from + minutes * 60 {
            if let Some(h) = e.tick(t, DAY, o.clone()) {
                out.push(h);
            }
            t += 30;
        }
        out
    }

    fn engine() -> Engine {
        Engine::new(Settings::default(), State::default())
    }

    #[test]
    fn levels() {
        let b = Band::new(2.0, 4.0);
        assert_eq!(Level::of(2.0, b), Level::Green);
        assert_eq!(Level::of(3.0, b), Level::Yellow);
        assert_eq!(Level::of(5.0, b), Level::Red);
    }

    #[test]
    fn first_hints_are_posture_then_water() {
        let mut e = engine();
        let hints = run(&mut e, T0, 50, &obs(1));
        let kinds: Vec<_> = hints.iter().map(|h| h.kind).collect();
        assert_eq!(kinds.first(), Some(&HintKind::Posture), "{kinds:?}");
        assert!(kinds.contains(&HintKind::Water), "{kinds:?}");
        assert!(
            hints.windows(2).all(|w| w[1].at - w[0].at >= 180),
            "Mindestabstand"
        );
    }

    #[test]
    fn drinking_and_posture_change_reset_their_reminders() {
        let mut e = engine();
        e.tick(T0, DAY, obs(1));
        e.drink(T0 + 60, None);
        assert_eq!(e.state.water_ml, 250);
        e.set_posture(T0 + 60, Posture::Standing);
        let hints = run(&mut e, T0 + 60, 14, &obs(1));
        assert!(hints.is_empty(), "{hints:?}");
        let hints = run(&mut e, T0 + 15 * 60, 3, &obs(1));
        assert_eq!(hints[0].kind, HintKind::Posture);
        assert!(hints[0].title.contains("wechseln"));
    }

    #[test]
    fn idle_counts_as_break_and_resets_block() {
        let mut e = engine();
        run(&mut e, T0, 70, &obs(1));
        assert!(e.snapshot(T0 + 70 * 60).block_min >= 69);
        // 12 Minuten weg
        let back = T0 + 82 * 60;
        e.tick(T0 + 75 * 60, DAY, obs(300));
        e.tick(back, DAY, obs(720));
        e.tick(back + 30, DAY, obs(2));
        let snap = e.snapshot(back + 30);
        assert_eq!(snap.breaks_today, 1);
        assert!(snap.block_min <= 1, "{}", snap.block_min);
        assert_eq!(snap.long_breaks_today, 0);
    }

    #[test]
    fn short_idle_is_no_break() {
        let mut e = engine();
        e.tick(T0, DAY, obs(1));
        e.tick(T0 + 300, DAY, obs(250));
        e.tick(T0 + 330, DAY, obs(1));
        assert_eq!(e.state.breaks.len(), 0);
    }

    #[test]
    fn hard_break_and_long_break_hints() {
        let mut e = engine();
        // Haltung & Wasser ruhigstellen, damit nur Pausenlogik greift
        e.settings.sit_minutes = 999;
        e.settings.water_target_ml = 0;
        let hints = run(&mut e, T0, 125, &obs(1));
        let kinds: Vec<_> = hints.iter().map(|h| (h.kind, h.level)).collect();
        assert!(
            kinds.contains(&(HintKind::Break, Level::Yellow)),
            "{kinds:?}"
        );
        assert!(kinds.contains(&(HintKind::Break, Level::Red)), "{kinds:?}");
        assert!(
            kinds.contains(&(HintKind::LongBreak, Level::Red)),
            "{kinds:?}"
        );
    }

    #[test]
    fn meetings_suppress_hints_and_idle_is_not_a_break() {
        let mut e = engine();
        e.tick(T0, DAY, obs(1));
        let mut meeting = obs(1500);
        meeting.camera_active = true;
        let hints = run(&mut e, T0 + 30, 60, &meeting);
        assert!(hints.is_empty(), "{hints:?}");
        assert_eq!(e.state.breaks.len(), 0);
        assert!(e.state.active_secs >= 59 * 60);
        // Kurz nach dem Meeting noch Ruhe, dann kommen die Hinweise
        assert!(e.tick(T0 + 61 * 60, DAY, obs(1)).is_none());
        let after = run(&mut e, T0 + 62 * 60, 3, &obs(1));
        assert!(!after.is_empty());
    }

    #[test]
    fn short_mic_use_is_dictation_not_meeting() {
        let mut e = engine();
        let mut dictating = obs(1);
        dictating.mic_active = true;
        e.tick(T0, DAY, dictating.clone());
        e.tick(T0 + 60, DAY, dictating.clone());
        assert!(!e.state.in_meeting);
        e.tick(T0 + 150, DAY, dictating);
        assert!(e.state.in_meeting);
    }

    #[test]
    fn load_turns_red_and_warns_after_hold() {
        let mut e = engine();
        e.settings.sit_minutes = 999;
        e.settings.water_target_ml = 0;
        let mut o = obs(1);
        o.tabs = (0..6)
            .map(|i| tab(&format!("t{i}"), &format!("~/p{i}"), i < 4))
            .collect();
        e.tick(T0, DAY, o.clone());
        let (level, metrics, reasons) = e.assess(T0);
        assert_eq!(level, Level::Red);
        assert!(metrics
            .iter()
            .any(|m| m.key == "agents" && m.level == Level::Red));
        assert!(!reasons.is_empty());
        let hints = run(&mut e, T0 + 30, 6, &o);
        assert_eq!(hints.first().map(|h| h.kind), Some(HintKind::Load));
    }

    #[test]
    fn counts_tab_switches_in_window() {
        let mut e = engine();
        for i in 0..8 {
            let mut o = obs(1);
            o.focused_key = Some(format!("w:{}", i % 2));
            e.tick(T0 + i * 20, DAY, o);
        }
        assert_eq!(e.state.switches.len(), 7);
        e.tick(T0 + 1000, DAY, obs(1));
        assert_eq!(e.state.switches.len(), 0);
    }

    #[test]
    fn sleep_gap_counts_as_break_and_new_day_rolls_over() {
        let mut e = engine();
        run(&mut e, T0, 30, &obs(1));
        e.tick(T0 + 30 * 60 + 1800, DAY, obs(1));
        assert_eq!(e.state.breaks.len(), 1);
        e.drink(T0 + 3000, None);
        e.tick(T0 + 86_400, "2026-10-24", obs(1));
        assert_eq!(e.state.water_ml, 0);
        assert_eq!(e.state.history.len(), 1);
        assert_eq!(e.state.history[0].water_ml, 250);
    }

    #[test]
    fn snooze_blocks_everything() {
        let mut e = engine();
        e.tick(T0, DAY, obs(1));
        e.snooze(T0, 60);
        assert!(run(&mut e, T0 + 30, 59, &obs(1)).is_empty());
    }

    #[test]
    fn manual_break_counts_and_pauses_hints() {
        let mut e = engine();
        run(&mut e, T0, 40, &obs(1));
        e.start_break(T0 + 40 * 60);
        assert!(run(&mut e, T0 + 40 * 60, 5, &obs(1)).is_empty());
        e.finish_break(T0 + 45 * 60);
        assert_eq!(e.state.breaks.len(), 1);
        assert_eq!(e.snapshot(T0 + 45 * 60).block_min, 0);
    }

    #[test]
    fn state_roundtrips_through_json() {
        let mut e = engine();
        run(&mut e, T0, 10, &obs(1));
        let json = serde_json::to_string(&e.state).unwrap();
        let back: State = serde_json::from_str(&json).unwrap();
        assert_eq!(back, e.state);
        let partial: Settings = serde_json::from_str(r#"{"water_target_ml": 2500}"#).unwrap();
        assert_eq!(partial.water_target_ml, 2500);
        assert_eq!(partial.glass_ml, 250);
        assert!(
            !partial.sound_enabled,
            "bestehende Einstellungen bleiben standardmäßig ohne Ton"
        );
    }
}
