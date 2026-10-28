//! Ghostty-Anbindung über die offizielle AppleScript-API (ab Ghostty 1.3).
//!
//! Objektmodell laut `Ghostty.sdef`: application → windows → tabs → terminals.
//! Pro Terminal liefert Ghostty `working directory`, `pid` (Vordergrundprozess)
//! und `tty` – damit lässt sich jedem Tab sein Projekt und laufender Agent zuordnen.

use serde::{Deserialize, Serialize};

/// Feldtrenner (ASCII Unit Separator) – kommt in Tab-Titeln praktisch nie vor.
pub const FIELD_SEP: char = '\u{1f}';

/// Liest alle Fenster/Tabs/Terminals in einem Rutsch aus.
///
/// Erste Zeile: `frontmost` der App. Danach eine Zeile pro Terminal:
/// `window-id ␟ tab-id ␟ tab-index ␟ tab-name ␟ selected ␟ is-front-window ␟ term-id ␟ cwd ␟ pid ␟ tty`
pub const SCRIPT: &str = r#"
set US to character id 31
set out to ""
tell application "Ghostty"
    set isFront to frontmost
    set out to (isFront as text) & linefeed
    set frontId to ""
    try
        set frontId to id of front window
    end try
    repeat with w in windows
        set wid to id of w
        set isFrontWin to (wid is frontId)
        repeat with t in tabs of w
            set tid to id of t
            set tidx to index of t
            set tname to ""
            try
                set tname to name of t
            end try
            set tsel to selected of t
            repeat with term in terminals of t
                set cwd to ""
                try
                    set cwd to working directory of term
                end try
                set tpid to ""
                try
                    set tpid to (pid of term) as text
                end try
                set ttty to ""
                try
                    set ttty to tty of term
                end try
                set out to out & wid & US & tid & US & (tidx as text) & US & tname & US & (tsel as text) & US & (isFrontWin as text) & US & (id of term) & US & cwd & US & tpid & US & ttty & linefeed
            end repeat
        end repeat
    end repeat
end tell
return out
"#;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Terminal {
    pub id: String,
    pub cwd: String,
    pub pid: Option<u32>,
    /// Kurzform wie in `ps` (z. B. `ttys016`)
    pub tty: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tab {
    pub window_id: String,
    pub id: String,
    pub index: u32,
    pub title: String,
    pub selected: bool,
    pub in_front_window: bool,
    pub terminals: Vec<Terminal>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GhosttyState {
    pub frontmost: bool,
    pub tabs: Vec<Tab>,
}

impl GhosttyState {
    /// Schlüssel des Tabs, auf den der Nutzer gerade schaut (nur wenn Ghostty vorne ist).
    pub fn focused_key(&self) -> Option<String> {
        if !self.frontmost {
            return None;
        }
        self.tabs
            .iter()
            .find(|t| t.in_front_window && t.selected)
            .map(|t| format!("{}:{}", t.window_id, t.id))
    }
}

fn short_tty(raw: &str) -> String {
    raw.trim().trim_start_matches("/dev/").to_string()
}

/// Parst die Ausgabe von [`SCRIPT`]. Mehrere Terminals (Splits) eines Tabs
/// werden zu einem Tab zusammengefasst.
pub fn parse(output: &str) -> GhosttyState {
    let mut lines = output.lines();
    let frontmost = lines.next().map(|l| l.trim() == "true").unwrap_or(false);
    let mut tabs: Vec<Tab> = Vec::new();

    for line in lines {
        let f: Vec<&str> = line.split(FIELD_SEP).collect();
        if f.len() < 10 {
            continue;
        }
        let term = Terminal {
            id: f[6].to_string(),
            cwd: f[7].trim().to_string(),
            pid: f[8].trim().parse().ok(),
            tty: short_tty(f[9]),
        };
        let (wid, tid) = (f[0], f[1]);
        if let Some(tab) = tabs.iter_mut().find(|t| t.window_id == wid && t.id == tid) {
            tab.terminals.push(term);
        } else {
            tabs.push(Tab {
                window_id: wid.to_string(),
                id: tid.to_string(),
                index: f[2].trim().parse().unwrap_or(0),
                title: f[3].to_string(),
                selected: f[4].trim() == "true",
                in_front_window: f[5].trim() == "true",
                terminals: vec![term],
            });
        }
    }
    GhosttyState { frontmost, tabs }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(fields: &[&str]) -> String {
        fields.join(&FIELD_SEP.to_string())
    }

    #[test]
    fn parses_tabs_and_merges_splits() {
        let out = [
            "true".to_string(),
            row(&[
                "w1",
                "t1",
                "1",
                "⠐ Refactor API",
                "true",
                "true",
                "a",
                "/Users/cz/dev/api",
                "4711",
                "/dev/ttys003",
            ]),
            row(&[
                "w1",
                "t1",
                "1",
                "⠐ Refactor API",
                "true",
                "true",
                "b",
                "/Users/cz/dev/api/web",
                "4712",
                "/dev/ttys004",
            ]),
            row(&[
                "w1",
                "t2",
                "2",
                "zsh",
                "false",
                "true",
                "c",
                "/Users/cz",
                "",
                "/dev/ttys005",
            ]),
            "kaputt".to_string(),
        ]
        .join("\n");

        let s = parse(&out);
        assert!(s.frontmost);
        assert_eq!(s.tabs.len(), 2);
        assert_eq!(s.tabs[0].terminals.len(), 2);
        assert_eq!(s.tabs[0].terminals[0].tty, "ttys003");
        assert_eq!(s.tabs[0].terminals[0].pid, Some(4711));
        assert_eq!(s.tabs[1].terminals[0].pid, None);
        assert_eq!(s.focused_key().as_deref(), Some("w1:t1"));
    }

    #[test]
    fn no_focus_when_ghostty_in_background() {
        let out = format!(
            "false\n{}",
            row(&["w1", "t1", "1", "x", "true", "true", "a", "/", "1", "ttys1"])
        );
        assert_eq!(parse(&out).focused_key(), None);
    }

    #[test]
    fn empty_output() {
        let s = parse("");
        assert!(!s.frontmost);
        assert!(s.tabs.is_empty());
    }
}
