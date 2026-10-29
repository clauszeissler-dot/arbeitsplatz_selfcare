# Lotse – User guide

🇩🇪 [Deutsche Version](ANLEITUNG.md) · [Back to overview](../README.md)

Lotse helps you stay fresh during long working days with AI. This guide shows you, step by step,
how to set up and use Lotse.

## What you need

- A Mac with macOS 13 or newer.
- The **Ghostty** terminal in its latest version (at least 1.3). Lotse also runs without Ghostty, but then it cannot see your tabs.

## Installing

There are no ready-made downloads yet. Until then, you build Lotse once yourself. The first time takes about 10 minutes.

1. Install the tools if you don't have them yet:
   - Xcode Command Line Tools: `xcode-select --install`
   - Rust: see [rustup.rs](https://rustup.rs)
   - Node.js 22: see [nodejs.org](https://nodejs.org)
2. Download this repository and open the folder in your terminal.
3. Run:
   ```sh
   npm install
   npm run tauri build
   ```
4. The finished app is at `target/release/bundle/macos/Lotse.app`. Drag it into your **Applications** folder.

Once signed downloads are available, you just download the DMG file and drag Lotse into **Applications**.

## First start

1. Open Lotse. A coloured dot appears in the menu bar. When the cockpit is open, Lotse appears in the Dock and ⌘Tab. Closing the cockpit returns it to menu-bar mode.
2. macOS asks: **"Lotse wants to control Ghostty"**. Click **OK**. Lotse only reads; it changes nothing.
3. Click the dot, then **Cockpit öffnen** (open cockpit).
4. In the cockpit, click the gear icon at the top right and switch on **"Beim Anmelden starten"** (start at login).

## Everyday use

**The traffic light in the menu bar**

- **Green:** You are working at a pace you can keep up all day.
- **Yellow:** Still fine. Finish your current thread on purpose.
- **Red:** Too much at once. Park a project before mistakes happen.

The number next to the dot shows how many minutes you have worked without a break.

**Answering a reminder**

Reminders appear as a small card at the top right. You have two options:
- Click the orange button once you have done it, for example **"Getrunken"** (drank) or **"Stehe"** (standing).
- Click **"Gleich"** (later), and the reminder comes back later.

If you want, a brief system sound can accompany the visible card. Turn **"Play reminder sound"** on in Settings; reminders always remain visible.

The card disappears on its own after 40 seconds. If you move the mouse over it, it stays.

**Logging without a reminder**

Click the dot in the menu bar:
- **Glas getrunken:** counts one glass of water.
- **Position:** office chair (default), Swopper, standing, or balance board. After the configured interval, Lotse prompts you to change posture. Each person can add or remove local posture options in Settings.
- **Pause starten / beenden:** start or end a break while you stay at your desk.
- **Hinweise pausieren:** quiet time for 30 minutes, 1 hour or 2 hours.

**Breaks**

Lotse notices on its own when you have not touched mouse or keyboard for 10 minutes. That counts as a break.
Closing your Mac's lid counts too.

**Meetings**

If your camera is on, or your microphone for more than 2 minutes, no reminders appear.
Short dictation does not count as a meeting. After a meeting, Lotse waits one minute and then speaks up.

## Settings

In the cockpit, via the gear icon. For example, you can change:
- your water goal and glass size
- how long you sit, stand or use the board
- when breaks are due
- whether reminders also play a sound
- when the traffic light turns yellow or red

Click **Speichern** (save) at the end.

## What Lotse reads and stores

- **Reads:** the titles and folders of your Ghostty tabs, which programs are running and how busy they are, how long you have not typed, and whether a camera or microphone is in use anywhere.
- **Records nothing:** Lotse does not listen and sees no camera image. It only checks whether a device is on.
- **Stores:** your daily values and settings only on your Mac, in `~/Library/Application Support/de.kiaffairs.lotse/`.
- **No internet needed:** Lotse works offline and sends nothing anywhere.

## Important limits

- Lotse only detects agents in **Ghostty** and the apps **ChatGPT** and **Claude Desktop**. It does not see what runs in your browser.
- Lotse only knows whether you sit or stand when you click it.
- Lotse estimates whether an agent is "working" from its CPU load. This can be wrong at times.
- If a program keeps the microphone on all the time, Lotse thinks you are always in a meeting. In that case, switch off **"Mikrofon zur Meeting-Erkennung nutzen"** (use microphone for meeting detection) in the settings.

## Help with problems

| Problem | Solution |
|---|---|
| The cockpit says "Ghostty-Zugriff fehlt" (Ghostty access missing) | System Settings → Privacy & Security → Automation → enable **Ghostty** for Lotse. |
| "Ghostty ist zu alt" (Ghostty too old) | Update Ghostty to the latest version. |
| No reminders at all | Check whether reminders are paused or a meeting is shown. |
| Reset today's values | Quit Lotse and delete `state.json` in the folder above. |
| Reset everything | Also delete `settings.json`. |

## Disclaimer

Lotse is a reminder tool. It is not a medical device and does not replace medical advice.
The default values are based on studies and recommendations ([sources, German](RECHERCHE.md)), but they do not fit everyone.
If you are in pain, talk to a doctor.


### Exercise timers and sound

Eye reminders offer a 20-second timer. Break reminders use your configured break duration. Click **Starten** to begin; the card stays visible until you finish or cancel it. **Klang zum Start und Abschluss** controls the start and completion sounds separately from automatic reminder sounds. Hear all three original, local sounds under **Lotse · Klangwelt** in Settings. Closing or cancelling a break timer ends the manually started break. Timers are not restored after quitting the app.
