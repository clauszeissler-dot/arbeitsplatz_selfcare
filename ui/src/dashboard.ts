import "@fontsource/inter/400.css";
import "@fontsource/inter/500.css";
import "@fontsource/inter/600.css";
import "@fontsource/space-grotesk/500.css";
import "@fontsource/space-grotesk/600.css";
import "@fontsource/space-grotesk/700.css";
import "./styles.css";
import "./dashboard.css";
import { api, listen, type Level, type Posture, type Settings, type Snapshot, type TabView } from "./api";
import { icons } from "./icons";

const app = document.getElementById("app")!;
let snap: Snapshot | null = null;
let settingsOpen = false;

const LEVEL_WORD: Record<Level, string> = { green: "GRÜN.", yellow: "GELB.", red: "ROT." };
const LEVEL_LEAD: Record<Level, string> = {
  green: "Du arbeitest in einem Tempo, das du den ganzen Tag halten kannst.",
  yellow: "Noch im Rahmen – aber der Kopf zahlt schon Zinsen. Nächsten Strang bewusst abschließen.",
  red: "Mehr parallel, als ein Kopf sauber überwachen kann. Park etwas, bevor Fehler entstehen.",
};
const POSTURES: { id: Posture; label: string }[] = [
  { id: "sitting", label: "Swopper" },
  { id: "standing", label: "Stehen" },
  { id: "board", label: "Board" },
];

function esc(s: string): string {
  return s.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]!);
}
const liters = (ml: number) => (ml / 1000).toLocaleString("de-DE", { minimumFractionDigits: 1, maximumFractionDigits: 1 });
const hm = (min: number) => `${Math.floor(min / 60)}:${String(Math.floor(min % 60)).padStart(2, "0")}`;
const pct = (a: number, b: number) => Math.max(0, Math.min(100, b > 0 ? (a / b) * 100 : 0));

function ring(value: number, max: number, level: Level, label: string, sub: string): string {
  const r = 52;
  const c = 2 * Math.PI * r;
  const p = pct(value, max) / 100;
  return `
    <div class="ring lvl-${level}">
      <svg viewBox="0 0 120 120">
        <circle cx="60" cy="60" r="${r}" class="track"/>
        <circle cx="60" cy="60" r="${r}" class="bar" stroke-dasharray="${c}" stroke-dashoffset="${c * (1 - p)}"/>
      </svg>
      <div class="ring-text"><strong>${label}</strong><span>${sub}</span></div>
    </div>`;
}

function trafficLight(level: Level): string {
  return `
    <div class="light" aria-label="Ampel ${level}">
      ${(["red", "yellow", "green"] as Level[]).map((l) => `<span class="lamp lvl-${l} ${l === level ? "on" : ""}"></span>`).join("")}
    </div>`;
}

function chips(s: Snapshot): string {
  const out: string[] = [];
  if (s.in_meeting) out.push(`<span class="chip cyan">${icons.meeting} Meeting – Hinweise ruhen</span>`);
  if (s.in_break) out.push(`<span class="chip cyan">${icons.pause} Pause läuft</span>`);
  if (s.snoozed_until) {
    const t = new Date(s.snoozed_until * 1000).toLocaleTimeString("de-DE", { hour: "2-digit", minute: "2-digit" });
    out.push(`<span class="chip">Hinweise pausiert bis ${t} <button class="link" data-act="resume">fortsetzen</button></span>`);
  }
  return out.join("");
}

function focusCard(s: Snapshot): string {
  const hard = 75;
  const level: Level = s.block_min >= 90 ? "red" : s.block_min >= 60 ? "yellow" : "green";
  const longLeft = Math.max(0, 120 - s.since_long_break_min);
  return `
    <section class="card">
      <div class="card-head"><span class="eyebrow">Fokus &amp; Pause</span>${icons.break}</div>
      <div class="split">
        ${ring(s.in_break ? 0 : s.block_min, hard, level, s.in_break ? "Pause" : `${s.block_min}′`, "am Stück")}
        <ul class="facts">
          <li><span>Nächste Pause</span><strong>${s.in_break ? "läuft" : s.next_break_min > 0 ? `in ${s.next_break_min} min` : "jetzt"}</strong></li>
          <li><span>Lotsen-Pause (30 min)</span><strong>${longLeft > 0 ? `in ${longLeft} min` : "fällig"}</strong></li>
          <li><span>Pausen heute</span><strong>${s.breaks_today} · davon lang ${s.long_breaks_today}</strong></li>
        </ul>
      </div>
      <div class="row">
        ${s.in_break
          ? `<button class="btn primary" data-act="break_end">${icons.pause} Pause beenden</button>`
          : `<button class="btn primary" data-act="break_start">${icons.break} Pause starten</button>`}
      </div>
    </section>`;
}

function waterCard(s: Snapshot): string {
  const glasses = Math.round(s.water_target_ml / s.glass_ml);
  const had = Math.floor(s.water_ml / s.glass_ml);
  const behind = s.water_expected_ml - s.water_ml >= 2 * s.glass_ml;
  return `
    <section class="card">
      <div class="card-head"><span class="eyebrow">Trinken</span>${icons.water}</div>
      <div class="big"><strong>${liters(s.water_ml)}</strong><span>/ ${liters(s.water_target_ml)} l</span></div>
      <div class="bar-track water">
        <div class="bar-fill" style="width:${pct(s.water_ml, s.water_target_ml)}%"></div>
        <div class="bar-mark" style="left:${pct(s.water_expected_ml, s.water_target_ml)}%" title="Soll jetzt"></div>
      </div>
      <div class="glasses">${Array.from({ length: glasses }, (_, i) => `<span class="${i < had ? "full" : ""}"></span>`).join("")}</div>
      <p class="note">${
        s.water_ml >= s.water_target_ml
          ? "Tagesziel erreicht."
          : behind
            ? `Du liegst ${liters(s.water_expected_ml - s.water_ml)} l hinter dem Plan – Erinnerungen kommen jetzt öfter.`
            : s.next_water_min === null
              ? ""
              : `Nächstes Glas ${s.next_water_min === 0 ? "jetzt" : `in ${s.next_water_min} min`}.`
      }</p>
      <div class="row"><button class="btn primary" data-act="drink">${icons.water} Glas getrunken (${s.glass_ml} ml)</button></div>
    </section>`;
}

function postureCard(s: Snapshot): string {
  const share = s.posture_share;
  const total = (share.sitting ?? 0) + (share.standing ?? 0) + (share.board ?? 0) || 1;
  const sit = pct(share.sitting ?? 0, total);
  const stand = pct((share.standing ?? 0) + (share.board ?? 0), total);
  const over = s.posture_min >= s.posture_limit_min;
  return `
    <section class="card">
      <div class="card-head"><span class="eyebrow">Haltung</span>${icons.posture}</div>
      <div class="big"><strong>${esc(s.posture_label)}</strong><span>${s.posture_min} / ${s.posture_limit_min} min</span></div>
      <div class="bar-track"><div class="bar-fill ${over ? "warn" : ""}" style="width:${pct(s.posture_min, s.posture_limit_min)}%"></div></div>
      <div class="seg" role="group" aria-label="Position">
        ${POSTURES.map((p) => `<button class="${p.id === s.posture ? "on" : ""}" data-act="posture_${p.id}">${p.label}</button>`).join("")}
      </div>
      <div class="split-bar" title="Heute: Sitzen / Stehen">
        <span class="sit" style="width:${sit}%"></span><span class="stand" style="width:${stand}%"></span>
      </div>
      <p class="note">Heute ${Math.round(sit)} % sitzend · ${Math.round(stand)} % stehend. Ziel: rund 60 / 30, dazu 10 % gehen.</p>
    </section>`;
}

function metricsGrid(s: Snapshot): string {
  return `
    <section class="metrics">
      ${s.metrics
        .map(
          (m) => `
        <div class="metric lvl-${m.level}" title="${esc(m.hint)}">
          <div class="metric-top"><span class="dot"></span><span class="mono">${esc(m.label)}</span></div>
          <strong>${esc(m.display)}</strong>
          <p>${esc(m.hint)}</p>
        </div>`,
        )
        .join("")}
    </section>`;
}

// Spinner-/Statuszeichen der Agenten vorne im Titel – der Status steht schon im Badge
const cleanTitle = (title: string) => title.replace(/^[⠀-⣿✳✶✻✽✢·*]\s*/u, "");

function agentBadges(t: TabView): string {
  return t.agents
    .map((a) => `<span class="agent ${a.busy ? "busy" : ""}">${a.busy ? '<i class="pulse"></i>' : ""}${esc(a.name)}</span>`)
    .join("");
}

function workList(s: Snapshot): string {
  const o = s.observation;
  const groups = new Map<string, TabView[]>();
  for (const t of o.tabs) {
    const k = t.project ?? "Ohne Projekt";
    groups.set(k, [...(groups.get(k) ?? []), t]);
  }
  const sorted = [...groups.entries()].sort(
    (a, b) => Number(b[1].some((t) => t.agents.some((x) => x.busy))) - Number(a[1].some((t) => t.agents.some((x) => x.busy))) || b[1].length - a[1].length,
  );
  const apps = [...o.apps, ...o.other_agents.map((a) => ({ name: a.name, busy: a.busy, cpu: a.cpu, sessions: 0 }))];

  let body: string;
  if (o.ghostty_note) body = `<p class="empty warn">${esc(o.ghostty_note)}</p>`;
  else if (!o.ghostty_running) body = `<p class="empty">Ghostty läuft gerade nicht.</p>`;
  else
    body = sorted
      .map(
        ([project, tabs]) => `
      <div class="project">
        <div class="project-head">
          <strong>${esc(project)}</strong>
          <span class="mono">${tabs.length} Tab${tabs.length === 1 ? "" : "s"}</span>
        </div>
        ${tabs
          .map(
            (t) => `
          <div class="tab ${t.focused ? "focused" : ""}">
            ${icons.terminal}
            <span class="tab-title">${esc(cleanTitle(t.title) || "Ohne Titel")}</span>
            <span class="badges">${agentBadges(t)}</span>
          </div>`,
          )
          .join("")}
      </div>`,
      )
      .join("");

  return `
    <section class="card wide">
      <div class="card-head">
        <span class="eyebrow">Was gerade läuft</span>
        <span class="mono">${o.tabs.length} Tabs · ${s.metrics.find((m) => m.key === "projects")?.display ?? 0} Projekte</span>
      </div>
      ${apps.length ? `<div class="apps">${apps.map((a) => `<span class="agent ${a.busy ? "busy" : ""}">${a.busy ? '<i class="pulse"></i>' : ""}${esc(a.name)}${a.sessions ? ` · ${a.sessions} Session${a.sessions > 1 ? "s" : ""}` : ""}</span>`).join("")}</div>` : ""}
      <div class="projects">${body}</div>
    </section>`;
}

function history(s: Snapshot): string {
  const days = s.history.slice(-7).reverse();
  if (!days.length) return "";
  return `
    <section class="card wide">
      <div class="card-head"><span class="eyebrow">Letzte Tage</span></div>
      <table class="hist">
        <thead><tr><th>Tag</th><th>Aktiv</th><th>Wasser</th><th>Pausen</th><th>Stehen</th><th>Rot</th><th>Max. Projekte</th></tr></thead>
        <tbody>
          ${days
            .map((d) => {
              const tot = (d.posture_min.sitting ?? 0) + (d.posture_min.standing ?? 0) + (d.posture_min.board ?? 0) || 1;
              const stand = Math.round(pct((d.posture_min.standing ?? 0) + (d.posture_min.board ?? 0), tot));
              const date = new Date(d.day + "T12:00:00").toLocaleDateString("de-DE", { weekday: "short", day: "2-digit", month: "2-digit" });
              return `<tr><td>${date}</td><td>${hm(d.active_min)} h</td><td>${liters(d.water_ml)} l</td><td>${d.breaks} (${d.long_breaks} lang)</td><td>${stand} %</td><td>${d.red_min} min</td><td>${d.max_projects}</td></tr>`;
            })
            .join("")}
        </tbody>
      </table>
    </section>`;
}

function render() {
  if (!snap) return;
  const s = snap;
  const workPct = pct(s.active_min, s.workday_min);
  app.innerHTML = `
    <header class="top" data-tauri-drag-region>
      <div class="logo" data-tauri-drag-region>KI Aff<span>AI</span>rs <em>Lotse</em></div>
      <div class="chips">${chips(s)}</div>
      <div class="top-actions">
        <button class="btn ghost" data-act="refresh" title="Jetzt aktualisieren">${icons.refresh}</button>
        <button class="btn ghost" data-act="settings" title="Einstellungen">${icons.settings}</button>
      </div>
    </header>

    <main>
      <section class="hero lvl-${s.level}">
        ${trafficLight(s.level)}
        <div class="hero-text">
          <span class="eyebrow">Belastung jetzt</span>
          <h1>Ampel steht auf <span class="lvl-word">${LEVEL_WORD[s.level]}</span></h1>
          <p class="lead">${LEVEL_LEAD[s.level]}</p>
          ${s.reasons.length ? `<ul class="reasons">${s.reasons.map((r) => `<li>${esc(r)}</li>`).join("")}</ul>` : ""}
        </div>
        <div class="day">
          <span class="mono">Arbeitstag</span>
          <strong>${hm(s.active_min)} <small>/ ${hm(s.workday_min)} h</small></strong>
          <div class="bar-track"><div class="bar-fill ${workPct >= 100 ? "warn" : ""}" style="width:${workPct}%"></div></div>
        </div>
      </section>

      <div class="cards">${focusCard(s)}${waterCard(s)}${postureCard(s)}</div>
      ${metricsGrid(s)}
      ${workList(s)}
      ${history(s)}

      <footer>
        <span class="mono">Grundlagen</span>
        Fluglotsen-Pausenregel (max. 2 h, dann 30 min – NATS/ICAO) · Sitz-Steh-Rhythmus 30/15 (Griffith University 2025) ·
        „AI Brain Fry“ (BCG/HBR 2026) · Parallele Projekte (Clark &amp; Wheelwright) · Wasserbedarf (EFSA) · Mikropausen (Albulescu 2022)
      </footer>
    </main>
    <div id="drawer" class="${settingsOpen ? "open" : ""}"></div>`;
  if (settingsOpen) void renderSettings();
}

// ---------- Einstellungen ----------

type Field = { key: keyof Settings; label: string; unit: string; min: number; max: number; step?: number };
const FIELDS: { title: string; fields: Field[] }[] = [
  {
    title: "Trinken",
    fields: [
      { key: "water_target_ml", label: "Tagesziel", unit: "ml", min: 500, max: 5000, step: 250 },
      { key: "glass_ml", label: "Glasgröße", unit: "ml", min: 100, max: 1000, step: 50 },
      { key: "water_interval_min", label: "Erinnerung alle", unit: "min", min: 15, max: 120 },
    ],
  },
  {
    title: "Haltung",
    fields: [
      { key: "sit_minutes", label: "Sitzen (Swopper)", unit: "min", min: 10, max: 120 },
      { key: "stand_minutes", label: "Stehen", unit: "min", min: 5, max: 90 },
      { key: "board_minutes", label: "Balance Board", unit: "min", min: 2, max: 60 },
    ],
  },
  {
    title: "Pausen (Lotsen-Prinzip)",
    fields: [
      { key: "block_soft_min", label: "Pause ankündigen nach", unit: "min", min: 20, max: 180 },
      { key: "block_hard_min", label: "Pause fällig nach", unit: "min", min: 30, max: 240 },
      { key: "long_break_after_min", label: "Lange Pause spätestens nach", unit: "min", min: 60, max: 300 },
      { key: "break_min", label: "Zählt als Pause ab", unit: "min", min: 3, max: 30 },
      { key: "long_break_min", label: "Zählt als lange Pause ab", unit: "min", min: 10, max: 60 },
      { key: "workday_minutes", label: "Arbeitstag", unit: "min", min: 120, max: 720, step: 15 },
    ],
  },
  {
    title: "Hinweise",
    fields: [
      { key: "toast_seconds", label: "Hinweis sichtbar", unit: "s", min: 5, max: 300 },
      { key: "min_gap_secs", label: "Mindestabstand", unit: "s", min: 30, max: 1800, step: 30 },
      { key: "meeting_mic_secs", label: "Mikro an ⇒ Meeting nach", unit: "s", min: 30, max: 900, step: 30 },
    ],
  },
];
const BANDS: { key: keyof Settings["thresholds"]; label: string }[] = [
  { key: "projects", label: "Parallele Projekte" },
  { key: "busy_agents", label: "Arbeitende KI-Agenten" },
  { key: "tabs", label: "Offene Tabs" },
  { key: "block_minutes", label: "Minuten ohne Pause" },
  { key: "switches_10min", label: "Tab-Wechsel / 10 min" },
  { key: "day_hours", label: "Aktive Stunden" },
];

async function renderSettings() {
  const drawer = document.getElementById("drawer")!;
  const [s, auto] = await Promise.all([api.settings(), api.autostart()]);
  drawer.innerHTML = `
    <div class="scrim" data-act="settings"></div>
    <form class="panel" id="settings-form">
      <div class="panel-head">
        <div><span class="eyebrow">Einstellungen</span><h2>Dein <span class="accent">Rhythmus.</span></h2></div>
        <button type="button" class="btn ghost" data-act="settings">${icons.close}</button>
      </div>
      ${FIELDS.map(
        (g) => `
        <fieldset><legend>${g.title}</legend>
          ${g.fields
            .map(
              (f) => `<label><span>${f.label}</span><span class="inp"><input type="number" name="${f.key}" value="${s[f.key]}" min="${f.min}" max="${f.max}" step="${f.step ?? 1}"/><em>${f.unit}</em></span></label>`,
            )
            .join("")}
        </fieldset>`,
      ).join("")}
      <fieldset><legend>Schalter</legend>
        <label class="toggle"><span>Mikrofon zur Meeting-Erkennung nutzen</span><input type="checkbox" name="meeting_use_mic" ${s.meeting_use_mic ? "checked" : ""}/></label>
        <label class="toggle"><span>Erinnerungston abspielen</span><input type="checkbox" name="sound_enabled" ${s.sound_enabled ? "checked" : ""}/></label>
        <label class="toggle"><span>Augen-Mikropause (20-20-20)</span><input type="checkbox" name="eyes_enabled" ${s.eyes_enabled ? "checked" : ""}/></label>
        <label class="toggle"><span>Beim Anmelden starten</span><input type="checkbox" name="autostart" ${auto ? "checked" : ""}/></label>
      </fieldset>
      <fieldset class="bands"><legend>Ampel-Schwellen <small>grün bis · gelb bis</small></legend>
        ${BANDS.map(
          (b) => `<label><span>${b.label}</span><span class="pair">
            <input type="number" step="0.5" name="t_${b.key}_green" value="${s.thresholds[b.key].green}"/>
            <input type="number" step="0.5" name="t_${b.key}_yellow" value="${s.thresholds[b.key].yellow}"/></span></label>`,
        ).join("")}
      </fieldset>
      <div class="panel-foot">
        <button type="submit" class="btn primary">Speichern</button>
        <span class="saved mono" id="saved"></span>
      </div>
    </form>`;

  const form = document.getElementById("settings-form") as HTMLFormElement;
  form.addEventListener("submit", async (e) => {
    e.preventDefault();
    const fd = new FormData(form);
    const next: Settings = structuredClone(s);
    for (const g of FIELDS) for (const f of g.fields) (next[f.key] as number) = Number(fd.get(f.key));
    next.meeting_use_mic = fd.get("meeting_use_mic") === "on";
    next.sound_enabled = fd.get("sound_enabled") === "on";
    next.eyes_enabled = fd.get("eyes_enabled") === "on";
    for (const b of BANDS) {
      next.thresholds[b.key] = { green: Number(fd.get(`t_${b.key}_green`)), yellow: Number(fd.get(`t_${b.key}_yellow`)) };
    }
    await api.saveSettings(next);
    const wantAuto = fd.get("autostart") === "on";
    if (wantAuto !== auto) await api.setAutostart(wantAuto).catch(() => {});
    document.getElementById("saved")!.textContent = "Gespeichert";
  });
}

// ---------- Ereignisse ----------

app.addEventListener("click", (e) => {
  const t = (e.target as HTMLElement).closest<HTMLElement>("[data-act]");
  if (!t) return;
  const act = t.dataset.act!;
  if (act === "settings") {
    settingsOpen = !settingsOpen;
    render();
  } else if (act === "refresh") {
    void api.refresh();
  } else {
    void api.act(act);
  }
});
async function load() {
  snap = await api.snapshot();
  render();
}

void listen<Snapshot>("snapshot", (s) => {
  snap = s;
  // Offenes Einstellungsformular nicht beim Tippen überschreiben
  if (!settingsOpen) render();
});
void load();
