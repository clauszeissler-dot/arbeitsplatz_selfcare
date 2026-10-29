export type UpdateViewState =
  | { phase: "idle" | "checking" | "current" | "error"; currentVersion: string }
  | { phase: "available"; currentVersion: string; version: string; notes: string }
  | { phase: "downloading"; currentVersion: string; version: string; progress: number | null }
  | { phase: "installing" | "restarting"; currentVersion: string; version: string };

export type UpdateAction = "check" | "install" | null;

export interface UpdatePresentation {
  status: string;
  action: UpdateAction;
  actionLabel: string | null;
  disabled: boolean;
}

export function compactUpdateNotes(notes: string | undefined): string {
  return (notes ?? "").replace(/\s+/g, " ").trim().slice(0, 240);
}

export function updatePresentation(state: UpdateViewState): UpdatePresentation {
  switch (state.phase) {
    case "idle":
      return {
        status: `Installiert: Version ${state.currentVersion}`,
        action: "check",
        actionLabel: "Nach Updates suchen",
        disabled: false,
      };
    case "checking":
      return { status: "GitHub Releases werden geprüft …", action: null, actionLabel: null, disabled: true };
    case "current":
      return {
        status: `Version ${state.currentVersion} ist aktuell.`,
        action: "check",
        actionLabel: "Erneut prüfen",
        disabled: false,
      };
    case "available": {
      const note = compactUpdateNotes(state.notes);
      return {
        status: `Version ${state.version} ist verfügbar.${note ? ` ${note}` : ""}`,
        action: "install",
        actionLabel: `Version ${state.version} installieren`,
        disabled: false,
      };
    }
    case "downloading": {
      const progress = state.progress === null ? "" : ` ${Math.max(0, Math.min(100, Math.round(state.progress)))} %`;
      return { status: `Update wird geladen …${progress}`, action: null, actionLabel: null, disabled: true };
    }
    case "installing":
      return { status: "Signatur geprüft. Update wird installiert …", action: null, actionLabel: null, disabled: true };
    case "restarting":
      return { status: "Update installiert. Lotse startet neu …", action: null, actionLabel: null, disabled: true };
    case "error":
      return {
        status: "Update konnte nicht geprüft oder installiert werden.",
        action: "check",
        actionLabel: "Erneut versuchen",
        disabled: false,
      };
  }
}
