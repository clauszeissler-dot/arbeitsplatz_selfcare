import hint from "./audio/hint.wav?url";
import start from "./audio/start.wav?url";
import complete from "./audio/complete.wav?url";
import { invoke, inTauri } from "./api";

export type Sound = "hint" | "start" | "complete";
const files = { hint, start, complete };
let playing: HTMLAudioElement | undefined;
export async function playSound(name: Sound): Promise<void> {
  if (inTauri) return invoke<void>("play_sound", { name });
  playing?.pause();
  playing = new Audio(files[name]);
  await playing.play();
}
