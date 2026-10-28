//! Führt Ghostty-Tabs, Prozessliste und Systemzustand zu einer Beobachtung zusammen.

use crate::ghostty::GhosttyState;
use crate::procs::{AppActivity, ProcIndex};
use crate::settings::Settings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentInfo {
    pub name: String,
    pub busy: bool,
    pub cpu: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabView {
    pub key: String,
    pub title: String,
    pub project: Option<String>,
    pub cwd: String,
    pub focused: bool,
    pub agents: Vec<AgentInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppView {
    pub name: String,
    pub busy: bool,
    pub cpu: f32,
    /// Code-/Agenten-Sessions, die die App selbst gestartet hat
    pub sessions: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    pub ghostty_running: bool,
    /// Hinweis, falls Ghostty läuft, aber nicht abgefragt werden kann
    pub ghostty_note: Option<String>,
    pub tabs: Vec<TabView>,
    pub apps: Vec<AppView>,
    /// Agenten ohne Ghostty-Tab (anderes Terminal, Hintergrund)
    pub other_agents: Vec<AgentInfo>,
    pub focused_key: Option<String>,
    pub idle_secs: u64,
    pub mic_active: bool,
    pub camera_active: bool,
}

impl Observation {
    pub fn projects(&self) -> Vec<String> {
        let mut p: Vec<String> = self.tabs.iter().filter_map(|t| t.project.clone()).collect();
        p.sort();
        p.dedup();
        p
    }

    pub fn busy_agents(&self) -> usize {
        let tabs = self
            .tabs
            .iter()
            .flat_map(|t| &t.agents)
            .filter(|a| a.busy)
            .count();
        let other = self.other_agents.iter().filter(|a| a.busy).count();
        let apps = self.apps.iter().filter(|a| a.busy).count();
        tabs + other + apps
    }

    pub fn total_agents(&self) -> usize {
        self.tabs.iter().map(|t| t.agents.len()).sum::<usize>()
            + self.other_agents.len()
            + self.apps.iter().map(|a| a.sessions as usize).sum::<usize>()
    }
}

/// Claude Code & Co. zeigen beim Arbeiten einen Braille-Spinner im Titel.
pub fn title_shows_spinner(title: &str) -> bool {
    title
        .chars()
        .next()
        .map(|c| ('\u{2800}'..='\u{28FF}').contains(&c) && c != '\u{2800}')
        .unwrap_or(false)
}

/// Fasst Arbeitsverzeichnisse zu Projekten zusammen: Unterordner gehören zum
/// übergeordneten Projekt, das Home-Verzeichnis selbst ist kein Projekt.
pub fn project_roots(cwds: &[String], home: &str) -> Vec<String> {
    let home = home.trim_end_matches('/');
    let mut dirs: Vec<String> = cwds
        .iter()
        .map(|c| c.trim_end_matches('/').to_string())
        .filter(|c| !c.is_empty() && c != home && c != "/" && c != "/tmp" && c != "/private/tmp")
        .collect();
    dirs.sort();
    dirs.dedup();
    let mut roots: Vec<String> = Vec::new();
    for d in dirs {
        if !roots.iter().any(|r| d.starts_with(&format!("{r}/"))) {
            roots.push(d);
        }
    }
    roots
}

fn project_for(cwd: &str, roots: &[String]) -> Option<String> {
    let cwd = cwd.trim_end_matches('/');
    roots
        .iter()
        .find(|r| cwd == r.as_str() || cwd.starts_with(&format!("{r}/")))
        .cloned()
}

pub fn display_project(path: &str, home: &str) -> String {
    let home = home.trim_end_matches('/');
    match path.strip_prefix(home) {
        Some(rest) if !home.is_empty() => format!("~{rest}"),
        _ => path.to_string(),
    }
}

pub struct SystemInput {
    pub idle_secs: u64,
    pub mic_active: bool,
    pub camera_active: bool,
}

pub fn build(
    ghostty: Option<&GhosttyState>,
    procs: &ProcIndex,
    sys: SystemInput,
    settings: &Settings,
    home: &str,
) -> Observation {
    let mut agents = procs.agents();
    let mut tabs = Vec::new();

    if let Some(g) = ghostty {
        let cwds: Vec<String> = g
            .tabs
            .iter()
            .flat_map(|t| t.terminals.iter().map(|x| x.cwd.clone()))
            .collect();
        let roots = project_roots(&cwds, home);
        let focused = g.focused_key();

        for t in &g.tabs {
            let key = format!("{}:{}", t.window_id, t.id);
            let spinner = title_shows_spinner(&t.title);
            let mut tab_agents = Vec::new();
            for term in &t.terminals {
                let mut i = 0;
                while i < agents.len() {
                    if !term.tty.is_empty() && agents[i].tty == term.tty {
                        let a = agents.remove(i);
                        tab_agents.push(AgentInfo {
                            busy: a.cpu >= settings.agent_busy_cpu || spinner,
                            name: a.name,
                            cpu: a.cpu,
                        });
                    } else {
                        i += 1;
                    }
                }
            }
            let cwd = t
                .terminals
                .first()
                .map(|x| x.cwd.clone())
                .unwrap_or_default();
            let project = t
                .terminals
                .iter()
                .find_map(|x| project_for(&x.cwd, &roots))
                .map(|p| display_project(&p, home));
            tabs.push(TabView {
                focused: focused.as_deref() == Some(key.as_str()),
                key,
                title: t.title.clone(),
                project,
                cwd: display_project(&cwd, home),
                agents: tab_agents,
            });
        }
    }

    let mut apps: Vec<AppView> = procs
        .desktop_apps()
        .into_iter()
        .map(|AppActivity { name, cpu }| AppView {
            busy: cpu >= settings.app_busy_cpu,
            name,
            cpu,
            sessions: 0,
        })
        .collect();

    let mut other_agents = Vec::new();
    for a in agents {
        let busy = a.cpu >= settings.agent_busy_cpu;
        if let Some(host) = &a.host_app {
            if let Some(app) = apps.iter_mut().find(|x| &x.name == host) {
                app.sessions += 1;
                // Die App selbst zählt als arbeitend, wenn eine ihrer Sessions arbeitet
                app.busy |= busy;
                continue;
            }
        }
        other_agents.push(AgentInfo {
            name: a.name,
            busy,
            cpu: a.cpu,
        });
    }

    Observation {
        ghostty_running: ghostty.is_some(),
        ghostty_note: None,
        focused_key: tabs.iter().find(|t| t.focused).map(|t| t.key.clone()),
        tabs,
        apps,
        other_agents,
        idle_secs: sys.idle_secs,
        mic_active: sys.mic_active,
        camera_active: sys.camera_active,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ghostty::{Tab, Terminal};
    use crate::procs::parse_ps;

    fn term(cwd: &str, tty: &str) -> Terminal {
        Terminal {
            id: tty.into(),
            cwd: cwd.into(),
            pid: None,
            tty: tty.into(),
        }
    }

    fn tab(id: &str, title: &str, sel: bool, terms: Vec<Terminal>) -> Tab {
        Tab {
            window_id: "w".into(),
            id: id.into(),
            index: 1,
            title: title.into(),
            selected: sel,
            in_front_window: true,
            terminals: terms,
        }
    }

    #[test]
    fn roots_collapse_subdirs_and_skip_home() {
        let cwds = vec![
            "/Users/cz/dev/api".to_string(),
            "/Users/cz/dev/api/web/".to_string(),
            "/Users/cz".to_string(),
            "/Users/cz/dev/apix".to_string(),
        ];
        assert_eq!(
            project_roots(&cwds, "/Users/cz/"),
            vec!["/Users/cz/dev/api", "/Users/cz/dev/apix"]
        );
    }

    #[test]
    fn spinner_detection() {
        assert!(title_shows_spinner("⠋ Fixing tests"));
        assert!(!title_shows_spinner("✳ Claude Code"));
        assert!(!title_shows_spinner(""));
    }

    #[test]
    fn builds_observation() {
        let ps = "\
  10 1 0.0 ttys001 -zsh
  11 10 20.0 ttys001 claude
  20 1 0.0 ttys002 -zsh
  21 20 0.1 ttys002 codex
  30 1 0.0 ttys009 claude
  40 1 25.0 ?? /Applications/ChatGPT.app/Contents/MacOS/ChatGPT
";
        let idx = ProcIndex::new(parse_ps(ps));
        let g = GhosttyState {
            frontmost: true,
            tabs: vec![
                tab(
                    "1",
                    "⠐ Build",
                    true,
                    vec![term("/Users/cz/dev/a", "ttys001")],
                ),
                tab(
                    "2",
                    "Codex",
                    false,
                    vec![term("/Users/cz/dev/b/src", "ttys002")],
                ),
                tab("3", "zsh", false, vec![term("/Users/cz", "ttys003")]),
            ],
        };
        let sys = SystemInput {
            idle_secs: 3,
            mic_active: false,
            camera_active: false,
        };
        let o = build(Some(&g), &idx, sys, &Settings::default(), "/Users/cz");

        assert_eq!(o.projects(), vec!["~/dev/a", "~/dev/b/src"]);
        assert_eq!(o.tabs[0].agents.len(), 1);
        assert!(o.tabs[0].agents[0].busy);
        assert!(!o.tabs[1].agents[0].busy);
        assert_eq!(o.other_agents.len(), 1);
        assert_eq!(o.apps.len(), 1);
        assert!(o.apps[0].busy);
        // Claude (Tab 1) + ChatGPT; Codex idle, Agent auf ttys009 ohne CPU
        assert_eq!(o.busy_agents(), 2);
        assert_eq!(o.total_agents(), 3);
        assert_eq!(o.focused_key.as_deref(), Some("w:1"));
    }
}
