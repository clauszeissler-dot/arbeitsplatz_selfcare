//! Prozessliste (`ps -axo pid=,ppid=,pcpu=,tty=,args=`) und Erkennung von KI-Agenten.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const PS_ARGS: &[&str] = &["-axo", "pid=,ppid=,pcpu=,tty=,args="];

#[derive(Debug, Clone, PartialEq)]
pub struct Proc {
    pub pid: u32,
    pub ppid: u32,
    pub cpu: f32,
    /// `ttys003` oder `??` ohne Terminal
    pub tty: String,
    pub args: String,
}

pub fn parse_ps(output: &str) -> Vec<Proc> {
    output
        .lines()
        .filter_map(|line| {
            let mut it = line.split_whitespace();
            let pid = it.next()?.parse().ok()?;
            let ppid = it.next()?.parse().ok()?;
            let cpu = it.next()?.replace(',', ".").parse().ok()?;
            let tty = it.next()?.to_string();
            let args = it.collect::<Vec<_>>().join(" ");
            Some(Proc {
                pid,
                ppid,
                cpu,
                tty,
                args,
            })
        })
        .collect()
}

/// Kommandozeilen-Agenten, die in Terminals laufen.
const CLI_AGENTS: &[(&str, &str)] = &[
    ("claude", "Claude Code"),
    ("codex", "Codex"),
    ("gemini", "Gemini CLI"),
    ("aider", "Aider"),
    ("opencode", "OpenCode"),
    ("amp", "Amp"),
    ("cursor-agent", "Cursor Agent"),
];

/// Desktop-Apps, deren Aktivität (CPU) wir als „KI arbeitet gerade“ werten.
pub const DESKTOP_APPS: &[(&str, &str)] = &[
    ("/Claude.app/", "Claude Desktop"),
    (
        "/ChatGPT.app/Contents/Frameworks/Codex Framework.framework/",
        "Codex App",
    ),
    ("/ChatGPT.app/Contents/Resources/codex-cli/", "Codex App"),
    ("/ChatGPT.app/", "ChatGPT"),
    ("/Codex.app/", "Codex App"),
];

fn basename(s: &str) -> &str {
    s.rsplit('/').next().unwrap_or(s)
}

/// Erkennt einen CLI-Agenten an der Kommandozeile – auch wenn er über Node läuft
/// (`node …/@anthropic-ai/claude-code/cli.js`).
pub fn agent_name(args: &str) -> Option<&'static str> {
    // Pfade mit Leerzeichen („Application Support“): Programmteil bis zur ersten Option
    let exe = args.find(" -").map_or(args, |i| &args[..i]);
    if let Some((_, name)) = CLI_AGENTS.iter().find(|(bin, _)| basename(exe) == *bin) {
        return Some(name);
    }
    let mut words = args.split_whitespace();
    let first = basename(words.next()?);
    if let Some((_, name)) = CLI_AGENTS.iter().find(|(bin, _)| first == *bin) {
        return Some(name);
    }
    if matches!(first, "node" | "bun" | "deno" | "python" | "python3") {
        let script = words.next().unwrap_or("");
        if script.contains("claude-code") {
            return Some("Claude Code");
        }
        if script.contains("@openai/codex") {
            return Some("Codex");
        }
        let b = basename(script);
        return CLI_AGENTS
            .iter()
            .find(|(bin, _)| b == *bin)
            .map(|(_, n)| *n);
    }
    None
}

pub fn desktop_app(args: &str) -> Option<&'static str> {
    DESKTOP_APPS
        .iter()
        .find(|(path, _)| args.contains(path))
        .map(|(_, n)| *n)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentProc {
    pub pid: u32,
    pub name: String,
    pub tty: String,
    /// CPU des Agenten inkl. aller Kindprozesse (Tools, Builds, Tests …)
    pub cpu: f32,
    /// Läuft unter einer Desktop-App (z. B. Code-Session aus Claude Desktop)
    pub host_app: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppActivity {
    pub name: String,
    pub cpu: f32,
}

pub struct ProcIndex {
    procs: Vec<Proc>,
    by_pid: HashMap<u32, usize>,
    children: HashMap<u32, Vec<u32>>,
}

impl ProcIndex {
    pub fn new(procs: Vec<Proc>) -> Self {
        let by_pid = procs.iter().enumerate().map(|(i, p)| (p.pid, i)).collect();
        let mut children: HashMap<u32, Vec<u32>> = HashMap::new();
        for p in &procs {
            children.entry(p.ppid).or_default().push(p.pid);
        }
        Self {
            procs,
            by_pid,
            children,
        }
    }

    fn get(&self, pid: u32) -> Option<&Proc> {
        self.by_pid.get(&pid).map(|&i| &self.procs[i])
    }

    pub fn subtree_cpu(&self, pid: u32) -> f32 {
        let mut sum = 0.0;
        let mut stack = vec![pid];
        let mut guard = 0;
        while let Some(p) = stack.pop() {
            guard += 1;
            if guard > 10_000 {
                break;
            }
            if let Some(proc_) = self.get(p) {
                sum += proc_.cpu;
            }
            if let Some(c) = self.children.get(&p) {
                stack.extend(c.iter().copied());
            }
        }
        sum
    }

    fn has_agent_ancestor(&self, pid: u32) -> bool {
        let mut cur = self.get(pid).map(|p| p.ppid);
        let mut depth = 0;
        while let Some(p) = cur {
            if p <= 1 || depth > 64 {
                return false;
            }
            match self.get(p) {
                Some(pp) if agent_name(&pp.args).is_some() => return true,
                Some(pp) => cur = Some(pp.ppid),
                None => return false,
            }
            depth += 1;
        }
        false
    }

    fn host_app(&self, pid: u32) -> Option<String> {
        let mut cur = Some(pid);
        let mut depth = 0;
        while let Some(p) = cur {
            if p <= 1 || depth > 64 {
                return None;
            }
            let proc_ = self.get(p)?;
            if let Some(app) = desktop_app(&proc_.args) {
                return Some(app.to_string());
            }
            cur = Some(proc_.ppid);
            depth += 1;
        }
        None
    }

    /// Alle Agenten-Prozesse (nur die obersten – ein Agent, der Unteragenten
    /// startet, zählt einmal).
    pub fn agents(&self) -> Vec<AgentProc> {
        self.procs
            .iter()
            .filter_map(|p| {
                let name = agent_name(&p.args)?;
                if self.has_agent_ancestor(p.pid) {
                    return None;
                }
                Some(AgentProc {
                    pid: p.pid,
                    name: name.to_string(),
                    tty: p.tty.clone(),
                    cpu: self.subtree_cpu(p.pid),
                    host_app: self.host_app(p.pid),
                })
            })
            .collect()
    }

    /// CPU-Summe je Desktop-App (alle Helfer-/Renderer-Prozesse zusammen).
    pub fn desktop_apps(&self) -> Vec<AppActivity> {
        let mut out: Vec<AppActivity> = Vec::new();
        for p in &self.procs {
            if let Some(name) = desktop_app(&p.args) {
                // Agenten unterhalb der App zählen separat
                if agent_name(&p.args).is_some() {
                    continue;
                }
                match out.iter_mut().find(|a| a.name == name) {
                    Some(a) => a.cpu += p.cpu,
                    None => out.push(AppActivity {
                        name: name.to_string(),
                        cpu: p.cpu,
                    }),
                }
            }
        }
        out
    }

    pub fn is_running(&self, needle: &str) -> bool {
        self.procs.iter().any(|p| p.args.contains(needle))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PS: &str = "\
    1     0   0,0 ??       /sbin/launchd
  500     1   1.2 ??       /Applications/Ghostty.app/Contents/MacOS/ghostty
  600   500   0.0 ttys003  -zsh
  601   600  12.5 ttys003  claude --resume
  602   601  30.0 ttys003  cargo test
  603   601   0.5 ttys003  claude mcp serve
  700   500   0.0 ttys004  -zsh
  701   700   0.3 ttys004  node /opt/homebrew/lib/node_modules/@openai/codex/bin/codex.js
  800     1   4.0 ??       /Applications/Claude.app/Contents/MacOS/Claude
  801   800  15.0 ??       /Applications/Claude.app/Contents/Frameworks/Claude Helper (Renderer).app/Contents/MacOS/Claude Helper (Renderer)
  802   800   2.0 ??       /Users/cz/Library/Application Support/Claude/claude-code/2.1.0/claude --output-format stream-json
  900     1   3.0 ??       /Applications/ChatGPT.app/Contents/MacOS/ChatGPT
";

    #[test]
    fn parses_ps_with_decimal_comma() {
        let p = parse_ps(PS);
        assert_eq!(p.len(), 12);
        assert_eq!(p[3].cpu, 12.5);
        assert_eq!(p[1].cpu, 1.2);
        assert_eq!(p[3].args, "claude --resume");
    }

    #[test]
    fn detects_top_level_agents_with_subtree_cpu() {
        let idx = ProcIndex::new(parse_ps(PS));
        let agents = idx.agents();
        let names: Vec<_> = agents
            .iter()
            .map(|a| (a.name.as_str(), a.tty.as_str()))
            .collect();
        assert_eq!(
            names,
            vec![
                ("Claude Code", "ttys003"),
                ("Codex", "ttys004"),
                ("Claude Code", "??")
            ]
        );
        assert!((agents[0].cpu - 43.0).abs() < 0.01);
        assert_eq!(agents[2].host_app.as_deref(), Some("Claude Desktop"));
        assert_eq!(agents[0].host_app, None);
    }

    #[test]
    fn sums_desktop_app_cpu_without_agents() {
        let idx = ProcIndex::new(parse_ps(PS));
        let apps = idx.desktop_apps();
        assert_eq!(apps.len(), 2);
        assert_eq!(apps[0].name, "Claude Desktop");
        assert!((apps[0].cpu - 19.0).abs() < 0.01);
        assert_eq!(apps[1].name, "ChatGPT");
    }

    #[test]
    fn agent_name_rejects_lookalikes() {
        assert_eq!(agent_name("/usr/bin/claudette"), None);
        assert_eq!(agent_name("vim claude.md"), None);
        assert_eq!(
            agent_name("/Users/x/.local/bin/claude"),
            Some("Claude Code")
        );
        assert_eq!(
            agent_name("node /usr/local/lib/node_modules/@anthropic-ai/claude-code/cli.js"),
            Some("Claude Code")
        );
    }

    #[test]
    fn recognizes_embedded_codex_desktop_components() {
        assert_eq!(
            desktop_app(
                "/Applications/ChatGPT.app/Contents/Frameworks/Codex Framework.framework/Helpers/Codex (Renderer).app/Contents/MacOS/Codex (Renderer)"
            ),
            Some("Codex App")
        );
    }
}
