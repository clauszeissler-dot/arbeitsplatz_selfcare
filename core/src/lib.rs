//! Lotse-Kern: alles, was ohne macOS-APIs testbar ist.
//!
//! - `ghostty`: AppleScript für Ghostty ≥ 1.3 und Parser für dessen Ausgabe
//! - `procs`: Parser für `ps`, Erkennung von KI-Agenten (Claude Code, Codex …)
//! - `idle`: Parser für die Leerlaufzeit aus `ioreg`
//! - `observe`: setzt Tabs + Prozesse zu einer Beobachtung zusammen
//! - `engine`: Ampel, Pausen-, Trink- und Haltungslogik

pub mod engine;
pub mod ghostty;
pub mod idle;
pub mod observe;
pub mod procs;
pub mod settings;

pub use engine::{Engine, Hint, HintKind, Level, Snapshot};
pub use observe::Observation;
pub use settings::{PostureDefinition, Settings};
