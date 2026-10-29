# Lotse – User guide

🇩🇪 [Deutsche Version](ANLEITUNG.md) · [Back to overview](../README.md)

Lotse helps you stay fresh during long working days with AI. This guide shows you, step by step,
how to set up and use Lotse.

## What you need

- A Mac with macOS 13 or newer.
- The **Ghostty** terminal in its latest version (at least 1.3). Lotse also runs without Ghostty, but then it cannot see your tabs.

## Installing

### Download the ready-to-use app

1. [Download Lotse 0.1.0 (.dmg)](https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/releases/download/lotse-0.1.0/Lotse.dmg) — for Apple Silicon (M1 or later), macOS 13 or later.
2. Open the DMG and drag **Lotse** into **Applications**.
3. Open **Lotse** from Applications. The package is Developer ID signed and notarized by Apple.

Version 0.1.0 does not contain the updater yet, so the first upgrade to the
updater-enabled 0.2.0 must be installed manually from its DMG. Starting with
0.2.0, Lotse checks the official GitHub release feed when the cockpit opens.
You can also use **Settings → Updates**. Download, installation, and restart only
begin after your click, and only correctly signed Universal packages are accepted.

### Build it yourself

Original code, documentation and Lotse audio are licensed under [Apache-2.0](../LICENSE).
You may build, modify and publish derivatives. Use your own bundle identifier for
custom builds; published modifications also need their own product name, branding
and developer identity. See [branding and identity rules](../BRANDING.md).

1. Install the tools if you don't have them yet:
   - Xcode Command Line Tools: `xcode-select --install`
   - Rust: see [rustup.rs](https://rustup.rs)
   - Node.js 22: see [nodejs.org](https://nodejs.org)
2. Download this repository and open the folder in your terminal.
3. Run:
   ```sh
   npm install
   npm run tauri -- build --no-sign --config '{"identifier":"com.yourname.breakhelper","productName":"BreakHelper"}'
   ```
4. The finished app is at `target/release/bundle/macos/BreakHelper.app`. Drag it into your **Applications** folder.

Replace the example identifier and name with your own. For a permanent fork,
update `src-tauri/tauri.conf.json` as well. Before publication, replace the original
branding and app icons and clearly identify modifications.

`--no-sign` does not produce a Developer ID signed or notarized package. Custom
builds do not inherit the original signature or notarization. Use your own Apple
credentials for distribution; see the [release guide](RELEASE.md).
A maintained fork must also remove the official Lotse updater configuration or
replace it with its own HTTPS feed and update signing key. The original private
update key is never distributed with this repository.

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

If you want, a soft original Lotse sound can accompany the card. Turn **"Sanfter Hinweisklang"** on in Settings. Timers have a separate sound switch; all three sounds can be previewed under **Lotse · Klangwelt**.

The card disappears on its own after 40 seconds. If you move the mouse over it, it stays.

**Logging without a reminder**

Click the dot in the menu bar:
- **Glas getrunken:** counts one glass of water.
- **Position:** office chair (default), Swopper, standing, or balance board. After the configured interval, Lotse prompts you to change posture. Each person can add or remove local posture options in Settings.
- **Pause starten / beenden:** start or end a break while you stay at your desk.
- **Today · tasks & breaks:** see when detected projects, apps, and agents started, how long they were active, and when breaks started and ended. Focus uses the traffic-light colors; an active break is consistently cyan.
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
- **Stores:** your daily values, settings, and today's task/break timeline only on your Mac, in `~/Library/Application Support/de.kiaffairs.lotse/`. The directory is restricted to your account (`0700`) and files to owner read/write (`0600`). Lotse retains at most 60 daily summaries; today's timeline is limited to 50 tasks and 50 breaks.
- **Not encrypted:** processes running as your signed-in user can generally read the local JSON files. Lotse does not persist tab titles, working directories, or full process arguments.
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
