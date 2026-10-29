import "@fontsource/inter/400.css";
import "@fontsource/inter/500.css";
import "@fontsource/space-grotesk/600.css";
import "@fontsource/space-grotesk/700.css";
import "./styles.css";
import "./toast.css";
import { api, inTauri, listen, mockToastHint, type Hint } from "./api";
import { playSound } from "./sounds";
import { icons, kindIcon } from "./icons";

const root = document.getElementById("toast")!;
let hideTimer: number | undefined;
let previewTick: number | undefined;
let toastSeconds = 40;
let current: Hint | null = null;
let running = false;
let finished = false;
let sound = true;
let starting = false;

function esc(s: string): string {
  return s.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]!);
}
function clock(s: number): string { return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`; }
function hide() {
  running = false;
  finished = false;
  current = null;
  window.clearInterval(previewTick);
  window.clearTimeout(hideTimer);
  root.classList.remove("in");
  // Cancel natively immediately; a closing animation must not leave a live timer behind.
  void api.hideToast();
}
function armTimer() {
  window.clearTimeout(hideTimer);
  if (running || finished || starting) return;
  hideTimer = window.setTimeout(hide, toastSeconds * 1000);
  const bar = root.querySelector<HTMLElement>(".timer");
  if (bar) {
    bar.style.animation = "none";
    void bar.offsetWidth;
    bar.style.animation = `drain ${toastSeconds}s linear forwards`;
  }
}
function updateTimer(remaining: number, done: boolean) {
  if (!current || (!running && !starting)) return;
  const time = root.querySelector<HTMLElement>(".countdown");
  const progress = root.querySelector<HTMLProgressElement>("progress");
  if (time) time.textContent = clock(remaining);
  if (progress) progress.value = (current.duration_seconds ?? 0) - remaining;
  if (!done) return;
  running = false;
  finished = true;
  window.clearInterval(previewTick);
  root.querySelector(".exercise-status")!.textContent = "Geschafft. Nimm dir ruhig noch einen Moment.";
  root.querySelector(".exercise-controls")!.innerHTML = `<button class="btn primary" data-done>Fertig</button>`;
  root.querySelector(".completion")!.textContent = "Die Zeit ist um.";
}
function render(h: Hint) {
  if (running || starting || finished) return;
  current = h;
  finished = false;
  root.className = `toast lvl-${h.level}`;
  const duration = h.duration_seconds;
  root.innerHTML = `
    <div class="edge"></div>
    <div class="icon">${kindIcon[h.kind] ?? icons.load}</div>
    <div class="content">
      <div class="head"><span class="brand">LOTSE</span>
        <button class="x" data-close aria-label="Schließen und Timer abbrechen">${icons.close}</button></div>
      <h1>${esc(h.title)}</h1><p>${esc(h.body)}</p>
      ${duration ? `<div class="exercise">
        <div class="exercise-top"><span class="exercise-status">Ein Moment für dich</span><span class="countdown" aria-label="Verbleibende Zeit">${clock(duration)}</span></div>
        <progress max="${duration}" value="0" aria-label="Übungsfortschritt"></progress>
        <label class="exercise-sound"><input type="checkbox" data-sound ${sound ? "checked" : ""}> Klang zum Start und Abschluss</label>
        <div class="exercise-controls"><button class="btn primary" data-start>Starten · ${duration < 60 ? `${duration} s` : `${duration / 60} min`}</button></div>
        <span class="completion" role="status" aria-live="polite"></span>
      </div>` : ""}
      <div class="actions">${h.actions.filter((a) => !duration || (a.id !== "break_start" && a.id !== "ok"))
        .map((a, i) => `<button class="btn ${i === 0 && !duration ? "primary" : ""}" data-action="${esc(a.id)}">${esc(a.label)}</button>`).join("")}</div>
      <span class="error" role="alert"></span>
    </div><div class="timer"></div>`;
  requestAnimationFrame(() => root.classList.add("in"));
  armTimer();
}
async function start() {
  if (running || starting || !current?.duration_seconds) return;
  starting = true;
  window.clearTimeout(hideTimer);
  const button = root.querySelector<HTMLButtonElement>("[data-start]")!;
  button.disabled = true;
  try {
    await api.startTimer(sound);
    running = true;
    root.querySelector(".timer")?.remove();
    root.querySelector(".actions")!.innerHTML = "";
    root.querySelector<HTMLInputElement>("[data-sound]")!.disabled = true;
    root.querySelector(".exercise-status")!.textContent = "Du kannst jetzt wegschauen.";
    root.querySelector(".exercise-controls")!.innerHTML = `<button class="btn" data-close>Timer abbrechen</button>`;
    if (!inTauri) {
      if (sound) void playSound("start").catch(() => {});
      const deadline = Date.now() + current.duration_seconds * 1000;
      previewTick = window.setInterval(() => {
        const seconds = Math.max(0, Math.ceil((deadline - Date.now()) / 1000));
        updateTimer(seconds, seconds === 0);
        if (seconds === 0 && sound) void playSound("complete").catch(() => {});
      }, 200);
    }
  } catch {
    button.disabled = false;
    root.querySelector(".error")!.textContent = "Timer konnte nicht gestartet werden. Bitte erneut versuchen.";
  } finally { starting = false; }
}
root.addEventListener("change", (e) => {
  if ((e.target as HTMLElement).matches("[data-sound]")) sound = (e.target as HTMLInputElement).checked;
});
root.addEventListener("click", (e) => {
  const t = (e.target as HTMLElement).closest<HTMLElement>("[data-action],[data-close],[data-start],[data-done]");
  if (!t) return;
  if (t.hasAttribute("data-start")) { void start(); return; }
  if (starting) return;
  if (t.dataset.action && t.dataset.action !== "ok") void api.act(t.dataset.action, current?.kind);
  hide();
});
root.addEventListener("mouseenter", () => {
  window.clearTimeout(hideTimer);
  root.querySelector<HTMLElement>(".timer")?.style.setProperty("animation-play-state", "paused");
});
root.addEventListener("mouseleave", armTimer);
void api.settings().then((s) => (toastSeconds = s.toast_seconds));
void listen<Hint>("hint", render);
void listen<{ remaining: number; finished: boolean }>("timer_update", (u) => updateTimer(u.remaining, u.finished));
if (!inTauri) render(mockToastHint());
