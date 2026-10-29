import { getVersion } from "@tauri-apps/api/app";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, type DownloadEvent, type Update } from "@tauri-apps/plugin-updater";

import { compactUpdateNotes, type UpdateViewState } from "./updater-model";

const UPDATE_TARGET = "macos-universal";
const UPDATE_CHECK_TIMEOUT_MS = 30_000;
const UPDATE_INSTALL_TIMEOUT_MS = 5 * 60_000;

export class LotseUpdater {
  state: UpdateViewState = { phase: "idle", currentVersion: "…" };
  private candidate: Update | null = null;

  constructor(private readonly onChange: () => void) {}

  private setState(state: UpdateViewState): void {
    this.state = state;
    this.onChange();
  }

  async check(): Promise<void> {
    if (["checking", "downloading", "installing", "restarting"].includes(this.state.phase)) return;
    const currentVersion = await getVersion().catch(() => this.state.currentVersion);
    this.setState({ phase: "checking", currentVersion });
    try {
      if (this.candidate) await this.candidate.close();
      this.candidate = await check({ target: UPDATE_TARGET, timeout: UPDATE_CHECK_TIMEOUT_MS });
      if (!this.candidate) {
        this.setState({ phase: "current", currentVersion });
        return;
      }
      this.setState({
        phase: "available",
        currentVersion,
        version: this.candidate.version,
        notes: compactUpdateNotes(this.candidate.body),
      });
    } catch {
      this.candidate = null;
      this.setState({ phase: "error", currentVersion });
    }
  }

  async install(): Promise<void> {
    if (!this.candidate || this.state.phase !== "available") return;
    const { currentVersion, version } = this.state;
    let downloaded = 0;
    let total: number | undefined;
    let lastProgress: number | null = null;
    this.setState({ phase: "downloading", currentVersion, version, progress: null });
    const report = (event: DownloadEvent) => {
      if (event.event === "Started") total = event.data.contentLength;
      if (event.event === "Progress") downloaded += event.data.chunkLength;
      const progress = total && total > 0 ? Math.min(100, (downloaded / total) * 100) : null;
      const rounded = progress === null ? null : Math.round(progress);
      if (rounded !== lastProgress) {
        lastProgress = rounded;
        this.setState({ phase: "downloading", currentVersion, version, progress });
      }
      if (event.event === "Finished") this.setState({ phase: "installing", currentVersion, version });
    };
    try {
      await this.candidate.downloadAndInstall(report, { timeout: UPDATE_INSTALL_TIMEOUT_MS });
      this.candidate = null;
      this.setState({ phase: "restarting", currentVersion, version });
      await relaunch();
    } catch {
      this.candidate = null;
      this.setState({ phase: "error", currentVersion });
    }
  }
}
