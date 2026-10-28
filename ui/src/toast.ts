import "@fontsource/inter/400.css";
import "@fontsource/inter/500.css";
import "@fontsource/space-grotesk/600.css";
import "@fontsource/space-grotesk/700.css";
import "./styles.css";
import "./toast.css";
import { api, inTauri, listen, mockToastHint, type Hint } from "./api";
import { icons, kindIcon } from "./icons";

const root = document.getElementById("toast")!;
let hideTimer: number | undefined;
let toastSeconds = 40;
let current: Hint | null = null;

function esc(s: string): string {
  return s.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]!);
}

function hide() {
  root.classList.remove("in");
  window.clearTimeout(hideTimer);
  window.setTimeout(() => void api.hideToast(), 220);
}

function armTimer() {
  window.clearTimeout(hideTimer);
  hideTimer = window.setTimeout(hide, toastSeconds * 1000);
  const bar = root.querySelector<HTMLElement>(".timer");
  if (bar) {
    bar.style.animation = "none";
    void bar.offsetWidth;
    bar.style.animation = `drain ${toastSeconds}s linear forwards`;
  }
}

function render(h: Hint) {
  current = h;
  root.className = `toast lvl-${h.level}`;
  root.innerHTML = `
    <div class="edge"></div>
    <div class="icon">${kindIcon[h.kind] ?? icons.load}</div>
    <div class="content">
      <div class="head">
        <span class="brand">LOTSE</span>
        <button class="x" data-close aria-label="Schließen">${icons.close}</button>
      </div>
      <h1>${esc(h.title)}</h1>
      <p>${esc(h.body)}</p>
      <div class="actions">
        ${h.actions
          .map((a, i) => `<button class="btn ${i === 0 ? "primary" : ""}" data-action="${esc(a.id)}">${esc(a.label)}</button>`)
          .join("")}
      </div>
    </div>
    <div class="timer"></div>`;
  requestAnimationFrame(() => root.classList.add("in"));
  armTimer();
}

root.addEventListener("click", (e) => {
  const t = (e.target as HTMLElement).closest<HTMLElement>("[data-action],[data-close]");
  if (!t) return;
  if (t.dataset.action && t.dataset.action !== "ok") {
    void api.act(t.dataset.action, current?.kind);
  }
  hide();
});

// Solange die Maus drauf ist, bleibt der Hinweis stehen
root.addEventListener("mouseenter", () => {
  window.clearTimeout(hideTimer);
  root.querySelector<HTMLElement>(".timer")?.style.setProperty("animation-play-state", "paused");
});
root.addEventListener("mouseleave", armTimer);

void api.settings().then((s) => (toastSeconds = s.toast_seconds));
void listen<Hint>("hint", render);

if (!inTauri) render(mockToastHint());
