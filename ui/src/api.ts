// Brücke zur Rust-Seite. Im normalen Browser (Designvorschau) liefert sie Beispieldaten.

export type Level = "green" | "yellow" | "red";
export type Posture = "sitting" | "standing" | "board";
export type HintKind = "long_break" | "break" | "load" | "posture" | "water" | "eyes" | "day_end";

export interface Metric {
  key: string;
  label: string;
  value: number;
  display: string;
  level: Level;
  hint: string;
}
export interface AgentInfo { name: string; busy: boolean; cpu: number }
export interface TabView {
  key: string;
  title: string;
  project: string | null;
  cwd: string;
  focused: boolean;
  agents: AgentInfo[];
}
export interface AppView { name: string; busy: boolean; cpu: number; sessions: number }
export interface Observation {
  ghostty_running: boolean;
  ghostty_note: string | null;
  tabs: TabView[];
  apps: AppView[];
  other_agents: AgentInfo[];
  focused_key: string | null;
  idle_secs: number;
  mic_active: boolean;
  camera_active: boolean;
}
export interface DaySummary {
  day: string;
  active_min: number;
  water_ml: number;
  breaks: number;
  long_breaks: number;
  posture_min: Partial<Record<Posture, number>>;
  red_min: number;
  max_projects: number;
  max_busy_agents: number;
}
export interface Snapshot {
  at: number;
  level: Level;
  metrics: Metric[];
  reasons: string[];
  in_meeting: boolean;
  in_break: boolean;
  snoozed_until: number | null;
  active_min: number;
  workday_min: number;
  block_min: number;
  since_long_break_min: number;
  water_ml: number;
  water_target_ml: number;
  water_expected_ml: number;
  glass_ml: number;
  posture: Posture;
  posture_label: string;
  posture_min: number;
  posture_limit_min: number;
  posture_share: Partial<Record<Posture, number>>;
  breaks_today: number;
  long_breaks_today: number;
  next_water_min: number | null;
  next_break_min: number;
  observation: Observation;
  history: DaySummary[];
}
export interface HintAction { id: string; label: string }
export interface Hint {
  kind: HintKind;
  level: Level;
  title: string;
  body: string;
  actions: HintAction[];
  at: number;
}
export interface Band { green: number; yellow: number }
export interface Settings {
  water_target_ml: number;
  glass_ml: number;
  water_interval_min: number;
  workday_minutes: number;
  sit_minutes: number;
  stand_minutes: number;
  board_minutes: number;
  block_soft_min: number;
  block_hard_min: number;
  long_break_after_min: number;
  break_min: number;
  long_break_min: number;
  idle_active_secs: number;
  meeting_mic_secs: number;
  meeting_use_mic: boolean;
  agent_busy_cpu: number;
  app_busy_cpu: number;
  toast_seconds: number;
  min_gap_secs: number;
  eyes_enabled: boolean;
  eyes_interval_min: number;
  thresholds: Record<"projects" | "tabs" | "busy_agents" | "block_minutes" | "switches_10min" | "day_hours", Band>;
}

export const inTauri = "__TAURI_INTERNALS__" in window;

type Unlisten = () => void;

async function tauriInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(cmd, args);
}

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (inTauri) return tauriInvoke<T>(cmd, args);
  return mock<T>(cmd, args);
}

export async function listen<T>(event: string, cb: (payload: T) => void): Promise<Unlisten> {
  if (inTauri) {
    const { getCurrentWebviewWindow } = await import("@tauri-apps/api/webviewWindow");
    // Ereignisse an alle Fenster und gezielt an dieses Fenster empfangen
    return getCurrentWebviewWindow().listen<T>(event, (e) => cb(e.payload));
  }
  mockListeners.push({ event, cb: cb as (p: unknown) => void });
  return () => {};
}

export const api = {
  snapshot: () => invoke<Snapshot>("get_snapshot"),
  settings: () => invoke<Settings>("get_settings"),
  saveSettings: (settings: Settings) => invoke<void>("save_settings", { settings }),
  act: (action: string, kind?: HintKind) => invoke<void>("act", { action, kind: kind ?? null }),
  hideToast: () => invoke<void>("hide_toast"),
  autostart: () => invoke<boolean>("autostart_enabled"),
  setAutostart: (enabled: boolean) => invoke<void>("set_autostart", { enabled }),
  refresh: () => invoke<void>("refresh"),
};

// ---------- Beispieldaten für die Browser-Vorschau ----------

const mockListeners: { event: string; cb: (p: unknown) => void }[] = [];

const defaultSettings: Settings = {
  water_target_ml: 2000, glass_ml: 250, water_interval_min: 45, workday_minutes: 480,
  sit_minutes: 30, stand_minutes: 15, board_minutes: 10, block_soft_min: 50, block_hard_min: 75,
  long_break_after_min: 120, break_min: 10, long_break_min: 25, idle_active_secs: 120,
  meeting_mic_secs: 120, meeting_use_mic: true, agent_busy_cpu: 3, app_busy_cpu: 10,
  toast_seconds: 40, min_gap_secs: 180, eyes_enabled: false, eyes_interval_min: 20,
  thresholds: {
    projects: { green: 2, yellow: 4 }, tabs: { green: 4, yellow: 8 }, busy_agents: { green: 2, yellow: 3 },
    block_minutes: { green: 60, yellow: 90 }, switches_10min: { green: 5, yellow: 15 }, day_hours: { green: 6, yellow: 8 },
  },
};

function mockSnapshot(): Snapshot {
  const tab = (n: number, title: string, project: string, agents: AgentInfo[], focused = false): TabView => ({
    key: `w1:${n}`, title, project, cwd: project, focused, agents,
  });
  const cc = (busy: boolean, cpu = 14): AgentInfo => ({ name: "Claude Code", busy, cpu });
  const tabs = [
    tab(1, "⠐ Refactor Abrechnung", "~/dev/abrechnung", [cc(true, 22)], true),
    tab(2, "✳ Tests Abrechnung", "~/dev/abrechnung", [cc(false, 0.4)]),
    tab(3, "⠂ Landingpage KI AffAIrs", "~/dev/ki-affairs-web", [cc(true, 31)]),
    tab(4, "codex – Datenimport", "~/dev/crm-import", [{ name: "Codex", busy: true, cpu: 9 }]),
    tab(5, "zsh", "~/dev/workshop-folien", []),
    tab(6, "✳ Recherche Förderung", "~/dev/foerderung", [cc(false, 0.2)]),
    tab(7, "zsh", "~/dev/lotse", []),
    ...Array.from({ length: 5 }, (_, i) => tab(8 + i, "zsh", "~/dev/kunde-" + "abcde"[i], [])),
  ];
  return {
    at: Date.now() / 1000, level: "red",
    metrics: [
      { key: "projects", label: "Parallele Projekte", value: 10, display: "10", level: "red", hint: "Optimum liegt bei 2 – ab 3 sinkt die wertschöpfende Zeit je Projekt deutlich." },
      { key: "agents", label: "KI arbeitet gerade", value: 4, display: "4 / 7", level: "red", hint: "Mehr als 2 gleichzeitig zu überwachende KI-Stränge erhöhen die mentale Ermüdung." },
      { key: "block", label: "Seit letzter Pause", value: 68, display: "68 min", level: "yellow", hint: "Fluglotsen: höchstens 2 h am Stück, dann 30 min ohne Verantwortung." },
      { key: "tabs", label: "Offene Tabs", value: 12, display: "12", level: "red", hint: "Jeder offene Tab ist ein offener Gedanke. Parken statt offen halten." },
      { key: "switches", label: "Tab-Wechsel / 10 min", value: 9, display: "9", level: "yellow", hint: "Jeder Wechsel hinterlässt Aufmerksamkeitsreste beim vorherigen Thema." },
      { key: "day", label: "Aktiv heute", value: 4.6, display: "4:36 h", level: "green", hint: "Kognitive Leistung sinkt am Ende langer Tage messbar." },
    ],
    reasons: ["Parallele Projekte: 10", "KI arbeitet gerade: 4 / 7", "Offene Tabs: 12"],
    in_meeting: false, in_break: false, snoozed_until: null,
    active_min: 276, workday_min: 480, block_min: 68, since_long_break_min: 104,
    water_ml: 750, water_target_ml: 2000, water_expected_ml: 1150, glass_ml: 250,
    posture: "sitting", posture_label: "Swopper", posture_min: 24, posture_limit_min: 30,
    posture_share: { sitting: 190, standing: 64, board: 12 },
    breaks_today: 2, long_breaks_today: 0, next_water_min: 6, next_break_min: 0,
    observation: {
      ghostty_running: true, ghostty_note: null, tabs,
      apps: [
        { name: "Claude Desktop", busy: false, cpu: 2.1, sessions: 1 },
        { name: "ChatGPT", busy: true, cpu: 18.4, sessions: 0 },
      ],
      other_agents: [], focused_key: "w1:1", idle_secs: 4, mic_active: false, camera_active: false,
    },
    history: [
      { day: "2026-10-20", active_min: 562, water_ml: 1000, breaks: 2, long_breaks: 0, posture_min: { sitting: 470, standing: 80, board: 12 }, red_min: 240, max_projects: 9, max_busy_agents: 5 },
      { day: "2026-10-21", active_min: 505, water_ml: 1500, breaks: 4, long_breaks: 1, posture_min: { sitting: 330, standing: 150, board: 25 }, red_min: 110, max_projects: 7, max_busy_agents: 4 },
      { day: "2026-10-22", active_min: 468, water_ml: 2000, breaks: 5, long_breaks: 2, posture_min: { sitting: 290, standing: 150, board: 28 }, red_min: 45, max_projects: 5, max_busy_agents: 3 },
    ],
  };
}

const mockHint: Hint = {
  kind: "water", level: "yellow", title: "Trinken",
  body: "0.8 von 2.0 l. Ein Glas (250 ml) jetzt – noch 1.3 l bis zum Ziel.",
  actions: [{ id: "drink", label: "Getrunken" }, { id: "snooze_10", label: "Gleich" }],
  at: Date.now() / 1000,
};

async function mock<T>(cmd: string, _args?: Record<string, unknown>): Promise<T> {
  switch (cmd) {
    case "get_snapshot": return mockSnapshot() as T;
    case "get_settings": return structuredClone(defaultSettings) as T;
    case "autostart_enabled": return true as T;
    default:
      if (cmd === "act") setTimeout(() => mockListeners.filter((l) => l.event === "snapshot").forEach((l) => l.cb(mockSnapshot())), 50);
      return undefined as T;
  }
}

export function mockToastHint(): Hint {
  const kind = new URLSearchParams(location.search).get("kind");
  if (kind === "break") {
    return {
      kind: "long_break", level: "red", title: "2 h ohne echte Pause",
      body: "Lotsen-Regel: 30 Minuten raus, ohne Verantwortung. Agenten laufen lassen oder parken, Bildschirm aus.",
      actions: [{ id: "break_start", label: "Pause starten" }, { id: "snooze_15", label: "In 15 min" }], at: Date.now() / 1000,
    };
  }
  if (kind === "posture") {
    return {
      kind: "posture", level: "yellow", title: "Aufstehen",
      body: "31 min gesessen. Jetzt 15 min stehen – gern ein paar davon aufs Board.",
      actions: [{ id: "posture_standing", label: "Stehe" }, { id: "posture_board", label: "Board" }], at: Date.now() / 1000,
    };
  }
  return mockHint;
}
