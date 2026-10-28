//! Einstellungen. Startwerte stammen aus der Recherche (siehe docs/RECHERCHE.md)
//! und sind bewusst alle anpassbar.

use serde::{Deserialize, Serialize};

/// Schwellen für eine Ampel-Kennzahl: `<= green` grün, `<= yellow` gelb, sonst rot.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Band {
    pub green: f64,
    pub yellow: f64,
}

impl Band {
    pub const fn new(green: f64, yellow: f64) -> Self {
        Self { green, yellow }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Thresholds {
    /// Parallele Projekte (Arbeitsverzeichnisse). Clark/Wheelwright: Optimum bei 2.
    pub projects: Band,
    /// Offene Ghostty-Tabs
    pub tabs: Band,
    /// Gleichzeitig arbeitende KI-Agenten. BCG 2026: ab 3 kippt es.
    pub busy_agents: Band,
    /// Minuten seit der letzten echten Pause. Fluglotsen: max. 120.
    pub block_minutes: Band,
    /// Tab-Wechsel in 10 Minuten (Attention Residue)
    pub switches_10min: Band,
    /// Aktive Stunden heute
    pub day_hours: Band,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            projects: Band::new(2.0, 4.0),
            tabs: Band::new(4.0, 8.0),
            busy_agents: Band::new(2.0, 3.0),
            block_minutes: Band::new(60.0, 90.0),
            switches_10min: Band::new(5.0, 15.0),
            day_hours: Band::new(6.0, 8.0),
        }
    }
}

/// Eine frei benennbare Haltung mit eigenem Wechselintervall. Sie wird nur
/// lokal in den Einstellungen der jeweiligen Person gespeichert.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostureDefinition {
    pub id: String,
    pub label: String,
    pub duration_min: u32,
}

pub fn default_postures() -> Vec<PostureDefinition> {
    vec![
        PostureDefinition {
            id: "office_chair".into(),
            label: "Bürostuhl".into(),
            duration_min: 30,
        },
        PostureDefinition {
            id: "swopper".into(),
            label: "Swopper".into(),
            duration_min: 30,
        },
        PostureDefinition {
            id: "standing".into(),
            label: "Stehen".into(),
            duration_min: 15,
        },
        PostureDefinition {
            id: "board".into(),
            label: "Balance Board".into(),
            duration_min: 10,
        },
    ]
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    // Trinken (EFSA: 2,0–2,5 l Gesamtwasser, ~80 % davon aus Getränken)
    pub water_target_ml: u32,
    pub glass_ml: u32,
    pub water_interval_min: u32,

    // Arbeitstag
    pub workday_minutes: u32,

    // Alte Einzelwerte bleiben beim Laden älterer lokaler Einstellungen erhalten.
    // Neue Einstellungen verwenden `postures` darunter.
    pub sit_minutes: u32,
    pub stand_minutes: u32,
    pub board_minutes: u32,
    #[serde(default)]
    pub postures: Vec<PostureDefinition>,

    // Pausen (Fluglotsen-Prinzip)
    pub block_soft_min: u32,
    pub block_hard_min: u32,
    pub long_break_after_min: u32,
    /// Ab so vielen Minuten ohne Eingabe zählt es als Pause (Albulescu 2022: >10 min)
    pub break_min: u32,
    /// Ab so vielen Minuten zählt es als „Lotsen-Pause“
    pub long_break_min: u32,

    // Erkennung
    /// Unter dieser Leerlaufzeit (Sekunden) gilt man als aktiv
    pub idle_active_secs: u32,
    /// Mikrofon so lange durchgehend an ⇒ Meeting (kurzes Diktieren zählt nicht)
    pub meeting_mic_secs: u32,
    /// Mikrofon zur Meeting-Erkennung nutzen (aus, falls eine App es dauerhaft belegt)
    pub meeting_use_mic: bool,
    /// CPU-Schwelle (%), ab der ein Agent als „arbeitet“ gilt
    pub agent_busy_cpu: f32,
    /// CPU-Schwelle (%) für Desktop-Apps (ChatGPT, Claude Desktop)
    pub app_busy_cpu: f32,

    // Darstellung
    pub toast_seconds: u32,
    /// Spielt bei einem neuen Hinweis den macOS-Systemton ab. Die Hinweis-Karte bleibt immer sichtbar.
    pub sound_enabled: bool,
    /// Mindestabstand zwischen zwei Hinweisen (Sekunden)
    pub min_gap_secs: u32,
    pub eyes_enabled: bool,
    pub eyes_interval_min: u32,

    pub thresholds: Thresholds,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            water_target_ml: 2000,
            glass_ml: 250,
            water_interval_min: 45,
            workday_minutes: 480,
            sit_minutes: 30,
            stand_minutes: 15,
            board_minutes: 10,
            postures: default_postures(),
            block_soft_min: 50,
            block_hard_min: 75,
            long_break_after_min: 120,
            break_min: 10,
            long_break_min: 25,
            idle_active_secs: 120,
            meeting_mic_secs: 120,
            meeting_use_mic: true,
            agent_busy_cpu: 3.0,
            app_busy_cpu: 10.0,
            toast_seconds: 40,
            sound_enabled: false,
            min_gap_secs: 180,
            eyes_enabled: false,
            eyes_interval_min: 20,
            thresholds: Thresholds::default(),
        }
    }
}

impl Settings {
    /// Vereinheitlicht Eingaben aus der lokalen Oberfläche und erzeugt bei
    /// älteren Einstellungsdateien die neue Liste aus deren bisherigen Werten.
    pub fn normalize_postures(&mut self) {
        if self.postures.is_empty() {
            self.postures = vec![
                PostureDefinition {
                    id: "office_chair".into(),
                    label: "Bürostuhl".into(),
                    duration_min: self.sit_minutes,
                },
                PostureDefinition {
                    id: "swopper".into(),
                    label: "Swopper".into(),
                    duration_min: self.sit_minutes,
                },
                PostureDefinition {
                    id: "standing".into(),
                    label: "Stehen".into(),
                    duration_min: self.stand_minutes,
                },
                PostureDefinition {
                    id: "board".into(),
                    label: "Balance Board".into(),
                    duration_min: self.board_minutes,
                },
            ];
        }

        let mut used = std::collections::BTreeSet::new();
        self.postures = std::mem::take(&mut self.postures)
            .into_iter()
            .filter_map(|mut posture| {
                posture.label = posture.label.trim().to_string();
                if posture.label.is_empty() {
                    return None;
                }
                let id: String = posture
                    .id
                    .to_ascii_lowercase()
                    .chars()
                    .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
                    .take(48)
                    .collect();
                if id.is_empty() || !used.insert(id.clone()) {
                    return None;
                }
                posture.id = id;
                posture.duration_min = posture.duration_min.clamp(1, 720);
                Some(posture)
            })
            .collect();
        if self.postures.is_empty() {
            self.postures.push(default_postures().remove(0));
        }
    }

    pub fn posture(&self, id: &str) -> Option<&PostureDefinition> {
        self.postures.iter().find(|posture| posture.id == id)
    }
}
