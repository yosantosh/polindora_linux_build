# Polindora — Complete App Documentation

> **Purpose**: Everything needed to rebuild Polindora from scratch. Every feature, UI element, setting, and how they connect.

---

## Table of Contents

1. [What Polindora Is](#1-what-polindora-is)
2. [Architecture](#2-architecture)
3. [File Structure](#3-file-structure)
4. [State Machine](#4-state-machine)
5. [Top Bar UI — Progress Bars](#5-top-bar-ui--progress-bars)
6. [Top Bar UI — Panel Indicator Icon](#6-top-bar-ui--panel-indicator-icon)
7. [Preferences Window — Home Page](#7-preferences-window--home-page)
8. [Preferences Window — Settings Pages](#8-preferences-window--settings-pages)
9. [Timer Logic](#9-timer-logic)
10. [Task Management](#10-task-management)
11. [Analytics](#11-analytics)
12. [Motivational Quotes](#12-motivational-quotes)
13. [Notifications & Sounds](#13-notifications--sounds)
14. [Themes](#14-themes)
15. [IPC: How the Two Processes Communicate](#15-ipc-how-the-two-processes-communicate)
16. [Complete GSettings Schema (30 Keys)](#16-complete-gsettings-schema-30-keys)
17. [CSS Classes Reference](#17-css-classes-reference)
18. [Icons (27 SVGs)](#18-icons-27-svgs)
19. [Installation](#19-installation)
20. [UI Element → Function Reference](#20-ui-element--function-reference)

---

## 1. What Polindora Is

Polindora is a Pomodoro timer App for GNOME Shell. It uses glassmorphism progress bars on the GNOME top panel — one bar for work (right of the clock), one for breaks (left of the clock). All configuration, task management, and analytics happen in a full-featured preferences window built with GTK4 and libadwaita.

**Core concept**: The Pomodoro technique cycles between focused work sessions and breaks. After a configurable number of work sessions, a longer break is given.

**Default cycle**: 25 min work → 5 min short break → 25 min work → 5 min short break → 25 min work → 5 min short break → 25 min work → 15 min long break → repeat.

**Design philosophy**: "Strict Focus" — by default, pause and skip buttons are hidden during work sessions (`strict-mode` = true), forcing the user to commit.

---

## 2. Architecture

Polindora is a **two-process system**:

```
┌─────────────────────────────────────────────────────────────┐
│                     GNOME Shell Process                      │
│                                                             │
│   extension.js                                              │
│   ┌──────────────────────────────────────────────────────┐  │
│   │  PomodoroBar (work)  │ CLOCK │  PomodoroBar (break)  │  │
│   │  Right of clock      │       │  Left of clock        │  │
│   └──────────────────────────────────────────────────────┘  │
│   ┌────────────────────┐                                    │
│   │  Panel Icon (♥)    │  ← Click opens prefs window       │
│   └────────────────────┘                                    │
│   Timer loop, state machine, notifications, sounds          │
│                                                             │
│                    ▲ GSettings (IPC) ▼                      │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                     GTK4 Process                            │
│                                                             │
│   prefs.js                                                  │
│   ┌──────────────────────────────────────────────────────┐  │
│   │  Home Page: Timer orb, controls, stats, tasks        │  │
│   │  Settings: Timer, Behavior, Appearance, Analytics,   │  │
│   │            Tasks, Theme (6 sub-pages)                │  │
│   └──────────────────────────────────────────────────────┘  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Key insight**: `extension.js` runs inside the GNOME Shell compositor. `prefs.js` runs as a separate GTK4 application. They communicate exclusively through GSettings keys — there is no direct function call between them.

All persistent data (analytics, tasks, settings, timer state) lives in GSettings. No external database, no files.

---

## 3. File Structure

| File | Purpose |
|---|---|
| `metadata.json` | App identity: UUID (`polindora@yosantosh.github.io`), name, GNOME versions (45–50), schema `org.gnome.shell.extensions.pomodoro-timer` |
| `extension.js` | Runs in GNOME Shell: timer loop, progress bars, panel icon, notifications, sounds, state machine |
| `prefs.js` | Runs as GTK4 app: full preferences window with timer control, analytics, task management, themes |
| `stylesheet.css` | All CSS for top panel bars, popup menu, and glassmorphism styling (2261 lines) |
| `quotes.js` | Array of 52 motivational quotes + `getRandomQuote()` and `getTimedQuote()` functions |
| `schemas/org.gnome.shell.extensions.pomodoro-timer.gschema.xml` | GSettings schema defining all 30 keys |
| `icons/` | 27 SVG icons for panel, tabs, and actions |
| `install.sh` | Installation script |
| `uninstall.sh` | Uninstallation script |

---

## 4. State Machine

Four timer states plus a paused overlay:

```
States:
  IDLE         — No timer running. Starting point.
  WORK         — Work session counting down.
  SHORT_BREAK  — Short break counting down.
  LONG_BREAK   — Long break counting down.

Overlay:
  _isPaused (boolean) — Can be true during WORK, SHORT_BREAK, or LONG_BREAK.
```

### State Transitions

```
                    ┌──────────────────────────────────────────┐
                    │                                          │
                    ▼                                          │
                 ┌──────┐   start-work     ┌──────┐           │
                 │ IDLE │────────────────►│ WORK │           │
                 └──────┘                 └──────┘           │
                    ▲                        │                │
                    │                        │ complete       │
                    │  reset / skip /        ▼                │
                    │  no auto-start    ┌───────────┐         │
                    │                   │ auto-start │         │
                    │                   │ breaks?    │         │
                    │                   └───────────┘         │
                    │                 yes│       │no          │
                    │                    ▼       └──►IDLE     │
                    │            ┌─────────────┐              │
                    │            │ SHORT_BREAK │              │
                    │            │   or        │              │
                    │            │ LONG_BREAK  │              │
                    │            └─────────────┘              │
                    │                    │ complete            │
                    │                    ▼                     │
                    │             ┌───────────┐               │
                    │             │ auto-start │               │
                    │             │ work?      │               │
                    │             └───────────┘               │
                    │           yes│       │no                │
                    │              └───────┘──────────────────┘
                    │                      │
                    │                      └───────►WORK (loop)
                    └──────────────────────────────────────────
```

### Long Break Logic

```javascript
const isLongBreak = (this._pomodoroCount % long_break_interval) === 0;
```

After every `long-break-interval` (default 4) work sessions, the break is a long break instead of a short one.

### Cycle Tracking

- `_pomodoroCount`: In-memory count of consecutive work sessions completed. Not persisted — resets on App disable/enable.
- `sessions-completed`: Persisted daily counter in GSettings.
- The session ring in the prefs Home page visually shows how many sessions are done within the current cycle.

---

## 5. Top Bar UI — Progress Bars

The progress bars are the most distinctive visual feature. They are custom `St.BoxLayout` widgets (`PomodoroBar` class) injected into the GNOME top panel flanking the system clock.

### Placement

- **Work bar**: Inserted **after** (right of) the `dateMenu` in the panel center box.
- **Break bar**: Inserted **before** (left of) the `dateMenu` in the panel center box.

### Structure (per bar)

Each `PomodoroBar` has this nested structure:

```
_outer  (St.Bin — glass capsule, translucent border, specular highlights)
  └─ _track  (St.Widget — dark inset background)
       ├─ _fill  (St.Widget — colored progress fill, animated width)
       └─ _shine  (St.Widget — top glass overlay / specular sheen)

_timeLabel  (St.Label — countdown text, e.g., "24:30")
```

### Dimensions & Appearance (from settings)

| Setting | Key | Default | Effect |
|---|---|---|---|
| Width | `bar-width` | 140 px | Total width of the bar capsule |
| Height | `bar-height` | 3 px | Height of the bar (schema range 2–8) |
| Corner Radius | `bar-radius` | 99 | 0 = rectangle, 99 = full capsule/pill |
| Saturation | `color-saturation` | 1.0 | Color saturation multiplier (0.0–2.0) |
| Glow Intensity | `glow-intensity` | 3 | Glow shadow spread in pixels |

### Progress Fill

- Progress fills **ascending** (0.0 → 1.0) as time elapses.
- `setProgress(fraction)` sets the fill width to `fraction × track_width`.
- At >80% progress during work, the fill CSS class switches to `pomodoro-work-fill-urgent` (amber/warning color).
- On completion, `flashComplete()` triggers an elastic bounce animation with a gold flash.

### Colors (from settings)

| State | Key | Default | CSS Fill Class |
|---|---|---|---|
| Work | `work-bar-color` | `#00d4ff` | `pomodoro-work-fill` |
| Work (urgent, >80%) | (same key) | (same) | `pomodoro-work-fill-urgent` |
| Break | `break-bar-color` | `#00ff66` | `pomodoro-break-fill` |
| Idle pulse | `idle-bar-color` | `#e6b800` | (breathing animation) |
| Complete flash | — | gold | `pomodoro-complete-fill` |

The `applyAppearance(hexColor, radius, saturation, glow)` method converts hex to RGB, adjusts saturation via HSL conversion, and applies inline styles + glow shadow.

### Animations

1. **Breathing Pulse** (idle state): Opacity oscillates 255↔130 using `Clutter.AnimationMode.EASE_IN_OUT_SINE`, 1800ms per cycle. Managed by `_startPulse()` / `_stopPulse()`.
2. **Progress Fill**: Smooth width transitions as the timer ticks every second.
3. **Completion Flash**: `flashComplete()` — elastic bounce effect using `Clutter.AnimationMode.EASE_OUT_ELASTIC`, gold color fill, then reset.

### Visibility

- Only the relevant bar is visible during a session:
  - WORK: Work bar visible, break bar hidden.
  - SHORT_BREAK / LONG_BREAK: Break bar visible, work bar hidden.
  - IDLE: Both bars can idle-pulse or hide.
- `showBar()` / `hideBar()` control visibility with opacity transitions.

### Time Label

- `St.Label` showing remaining time (e.g., `"24:30"`), positioned next to the bar.
- CSS class: `pomodoro-bar-time-label` / `pomodoro-bar-time-active`.
- Updated by `setTimeText(text)`.

### Bar Click Behavior

| Action | Handler |
|---|---|
| Single click on either bar | `_onBarClicked()` → Pause/Resume timer (writes `pause`/`resume` to `timer-command` GSettings key) |
| Double click (<300ms) on either bar | `_onBarClicked()` → Opens preferences window via `_openPreferences()` |

---

## 6. Top Bar UI — Panel Indicator Icon

A `PanelMenu.Button` placed in the GNOME status area (top-right tray).

### Icon

- Displays `polindora-logo.svg` as a 16px `St.Icon`.
- **The popup menu is completely suppressed**:
  - `menu.actor.hide()` makes the menu invisible.
  - `menu.toggle()` is replaced to call `_openPreferences()` instead.
  - `open-state-changed` listener force-closes the menu if anything opens it.
- **Left-clicking the icon opens the preferences window** (not a popup).

### Icon CSS State Classes

| State | CSS Class |
|---|---|
| Idle | `pomodoro-indicator-icon` (dimmed white) |
| Work | `pomodoro-indicator-icon-active` (bright white with glow) |
| Break | `pomodoro-indicator-icon-break` |

---

## 7. Preferences Window — Home Page

The Home page is the main interface of the App. It's a full app experience with a timer, controls, stats, and tasks — not just settings toggles.

### 7.1 Header Strip

- **Settings gear button** (⚙): Opens the Settings sub-window containing the 6 configuration pages.

### 7.2 Timer Display Orb

A large circular glassmorphism orb centered on the page:

- **Session Ring**: A segmented progress ring around the orb showing completed pomodoros vs. total (segments = `long-break-interval`). Completed segments are highlighted.
- **Heart Icon** (♥): Decorative icon inside the orb. Has a birthday Easter egg — on March 4, clicking/hovering shows "Happy Birthday P_S 🎂🎉" for 8 seconds (controlled by `bday-surprise` setting).
- **State Label**: Text above the timer digits:
  - `"FOCUS"` during work
  - `"BREAK"` during short break
  - `"LONG BREAK"` during long break
- **Timer Digits**: Large monospace countdown (`MM:SS`), e.g., `"24:30"`.
- **Motivation Quote**: Text below the timer that changes based on timer state and progress (from `quotes.js`).

### 7.3 Session Controls

| UI Element | Visible When | Action | IPC Command Written |
|---|---|---|---|
| **Category Dropdown** (`Gtk.DropDown`) | Always | Select focus category for current session | Writes to `timer-category` |
| **Start Focus Button** | IDLE | Start a work session | `'start-work'` |
| **Pause Button** | WORK/BREAK, running, `strict-mode` OFF | Pause the timer | `'pause'` |
| **Resume Button** | Paused | Resume the timer | `'resume'` |
| **Skip Button** | Running/Paused, `strict-mode` OFF | Skip current session | `'skip'` |
| **Reset Button** | Running/Paused | Reset timer to IDLE | `'reset'` |

> **Strict Mode**: When `strict-mode` is ON (default), the Pause and Skip buttons are **hidden** during work sessions. The user cannot interrupt their focus. These buttons only become visible during breaks or when strict mode is disabled.

### 7.4 Today Stats

Displayed below the controls:
- **Pomodoros Completed**: Reads `sessions-completed`.
- **Total Focus Time**: Reads `total-focus-minutes`.

### 7.5 Top Tasks

Shows the top 2 active (not done) tasks with:
- Task name and category
- **"Mark done" button**: Sets `task.done = true`, saves to `tasks` GSettings key.

### 7.6 Live Sync with the Timer

The Home page subscribes to GSettings `changed` signals on `timer-state`, `timer-is-paused`, `timer-start-time`, `timer-elapsed-accumulated`, and `timer-duration-secs`. It uses these to compute and display the real-time countdown:

1. Reads `timer-start-time` (monotonic timestamp) and `timer-elapsed-accumulated`.
2. Computes elapsed = `(GLib.get_monotonic_time() - timer_start_time) / 1e6 + elapsed_accumulated`.
3. Remaining = `timer-duration-secs - elapsed`.
4. Updates the timer digits and session ring every second.

---

## 8. Preferences Window — Settings Pages

Clicking the ⚙ gear on the Home page opens a sub-window with **6 pages**:

### Page 1: Timer

| Group | Setting | Widget | Key | Range | Default |
|---|---|---|---|---|---|
| Session Durations | Work Duration | `Adw.SpinRow` | `work-duration` | 1–60 min | 25 |
| | Short Break | `Adw.SpinRow` | `short-break-duration` | 1–30 min | 5 |
| | Long Break | `Adw.SpinRow` | `long-break-duration` | 1–60 min | 15 |
| | Long Break Interval | `Adw.SpinRow` | `long-break-interval` | 2–8 | 4 |

### Page 2: Behavior

| Group | Setting | Widget | Key | Default |
|---|---|---|---|---|
| Focus Enforcement | Strict Mode | `Adw.SwitchRow` | `strict-mode` | ON |
| | Auto-start Work | `Adw.SwitchRow` | `auto-start-work` | ON |
| | Auto-start Breaks | `Adw.SwitchRow` | `auto-start-breaks` | ON |
| Notifications | Desktop Notifications | `Adw.SwitchRow` | `show-notifications` | ON |
| | Sound Notifications | `Adw.SwitchRow` | `play-sound-notifications` | ON |

### Page 3: Appearance

| Group | Setting | Widget | Key | Range | Default |
|---|---|---|---|---|---|
| Bar Dimensions | Bar Width | `Adw.SpinRow` | `bar-width` | 60–300 px | 140 |
| | Bar Height | `Adw.SpinRow` | `bar-height` | 2–8 px | 3 |
| | Bar Corner Radius | `Adw.SpinRow` | `bar-radius` | 0–99 | 99 |
| Colors | Work Bar Color | `Gtk.ColorDialogButton` | `work-bar-color` | hex | `#00d4ff` |
| | Break Bar Color | `Gtk.ColorDialogButton` | `break-bar-color` | hex | `#00ff66` |
| | Idle Pulse Color | `Gtk.ColorDialogButton` | `idle-bar-color` | hex | `#e6b800` |
| | Color Saturation | `Adw.SpinRow` (0.1 step) | `color-saturation` | 0.0–2.0 | 1.0 |
| | Glow Intensity | `Adw.SpinRow` | `glow-intensity` | 0–20 | 3 |
| (Reset) | Reset Colors to Default | `Gtk.Button` | — | — | Resets 5 color keys to Default Glass preset |

### Page 4: Analytics (read-only dashboard)

| Group | Content |
|---|---|
| Today's Focus | Pomodoros completed, total focus time, avg session length, all-time focus minutes |
| Streak | Current streak (computed from `analytics-history`), best streak (`best-streak`) |
| Category Breakdown | **Donut pie chart** (220×220 `Gtk.DrawingArea`) showing time per category. Interactive: clicking a bar in the 7-day chart filters the pie to that day. Hover tooltips on slices. |
| Last 7 Days | **Bar chart** (396×190 `Gtk.DrawingArea`) showing daily focus minutes. Today's bar highlighted in pink. Clickable bars update the pie chart. Selected bar highlighted in gold. |
| Task Completion | Total tasks, completed, active, completion rate (uses `tasks` + `deleted-incomplete-count`) |
| Reset | **Reset Today's Stats** button (zeros `sessions-completed`, `total-focus-minutes`, `task-stats`). **Reset All History** button (zeros everything including `all-time-focus-minutes`, `analytics-history`, `best-streak`, `deleted-incomplete-count`). |

### Page 5: Tasks

| Group | Content |
|---|---|
| Task Overview | Total tasks, completed, remaining, completion rate (display only) |
| Create New Task | Task Name (`Adw.EntryRow`), Category (`Adw.EntryRow`, optional), Add Task button |
| Active Tasks | Dynamic rows per active task with ✓ Mark done button and 🗑 Delete button (delete shows confirmation dialog warning about completion rate impact) |
| Completed Tasks | Dynamic rows per completed task with ↩ Undo button (sends back to active) and 🗑 Delete button |

### Page 6: Theme

| Group | Setting | Widget | Key | Default |
|---|---|---|---|---|
| App Theme | Theme | `Adw.ComboRow` | `theme-name` | `'default'` |
| Custom Colors | Work Accent | `Gtk.ColorDialogButton` | `work-bar-color` | — |
| | Break Accent | `Gtk.ColorDialogButton` | `break-bar-color` | — |
| | Idle Accent | `Gtk.ColorDialogButton` | `idle-bar-color` | — |

Custom color pickers are active when theme is set to `'custom'`. Selecting any other theme auto-applies its preset colors.

**9 Theme Presets**:

| Theme ID | Label | Work Bar | Break Bar | Idle Bar |
|---|---|---|---|---|
| `default` | Default Glass | `#a3d5ff` | `#eaf2ff` | `#ffffff` |
| `black-pink` | Black Pink | `#ff69b4` | `#ff1493` | `#ffb6c1` |
| `aurora` | Aurora | `#72f2d1` | `#9da8ff` | `#d7fff6` |
| `emerald` | Emerald | `#51e69a` | `#bbf7d0` | `#e8fff1` |
| `amber` | Amber | `#fbbf24` | `#fde68a` | `#fff7d6` |
| `violet` | Violet | `#b794ff` | `#f0abfc` | `#f6e8ff` |
| `ocean` | Ocean | `#38bdf8` | `#67e8f9` | `#e0f7ff` |
| `graphite` | Graphite | `#cbd5e1` | `#94a3b8` | `#f8fafc` |
| `custom` | Custom | (user-defined) | (user-defined) | (user-defined) |

---

## 9. Timer Logic

### Location

All timer logic runs in `extension.js` inside the GNOME Shell process.

### Starting Work (`_startWork`)

1. Reads `work-duration` setting (1–60 min, default 25), converts to seconds.
2. Shows work bar, hides break bar.
3. Sets icon to `pomodoro-indicator-icon-active`.
4. Writes IPC state keys to GSettings: `timer-state='work'`, `timer-duration-secs`, `timer-start-time`, `timer-elapsed-accumulated=0`.
5. Starts `_startTimerLoop()`.

### Starting Break (`_startBreak(isLong)`)

1. If `isLong`: reads `long-break-duration` (1–60 min, default 15).
2. If short: reads `short-break-duration` (1–30 min, default 5).
3. Shows break bar, hides work bar.
4. Sets icon to `pomodoro-indicator-icon-break`.
5. Writes IPC state keys.
6. Starts `_startTimerLoop()`.

### Timer Loop (`_startTimerLoop`)

1. Creates a 1-second `GLib.timeout_add` interval.
2. Uses **monotonic time** (`GLib.get_monotonic_time()`) for accurate delta calculation (survives system clock changes and sleep).
3. Every tick:
   - Calculates elapsed seconds from monotonic delta.
   - Computes `progress = elapsed / totalDuration` (0.0 → 1.0, ascending).
   - Calls `bar.setProgress(progress)` to update the fill width.
   - Calls `bar.setTimeText(formatTime(remaining))` to update the time label.
   - At >80% progress during work: switches fill class to `pomodoro-work-fill-urgent`.
   - At completion (progress ≥ 1.0): calls `_onTimerComplete()`.

### Timer Completion (`_onTimerComplete`)

**If WORK completed**:
1. Increments `_pomodoroCount` (in-memory consecutive count).
2. Increments `sessions-completed` in GSettings (daily persistent).
3. Adds `work-duration` to `total-focus-minutes` (daily) and `all-time-focus-minutes` (lifetime).
4. Updates `task-stats` JSON: adds minutes to the current category.
5. Increments `pomodorosSpent` on all active (not done) tasks matching the current category.
6. Appends/updates today's entry in `analytics-history` (capped at 30 days).
7. Calls `flashComplete()` on work bar (gold elastic bounce animation).
8. Sends desktop notification: `"Pomodoro Complete!"` with session count + random streak motivation (if `show-notifications` = true).
9. Plays GNOME sound theme `'complete'` event (if `play-sound-notifications` = true).
10. After 2200ms delay:
    - If `auto-start-breaks` = true: auto-starts break. Uses `_pomodoroCount % long-break-interval === 0` to decide long vs short.
    - Otherwise: enters IDLE.

**If BREAK completed**:
1. Calls `flashComplete()` on break bar.
2. Notification: `"⏰ Break Over! — Time to get back to work. Stay focused!"`.
3. Plays sound.
4. After 2200ms delay:
    - If `auto-start-work` = true: auto-starts work.
    - Otherwise: enters IDLE.

### Pause/Resume (`_togglePause`)

1. Toggles `_isPaused` flag.
2. If pausing: records elapsed time into `timer-elapsed-accumulated`. Timer loop continues ticking but skips time accumulation.
3. If resuming: records new `timer-start-time` as current monotonic time.
4. Writes `timer-is-paused` to GSettings for prefs window sync.

### Skip (`_skip`)

- **If WORK**: Resets work bar, goes to IDLE. **Does NOT count as completed** — no analytics update.
- **If BREAK**: Resets break bar, starts work.

### Reset (`_reset`)

- Stops timer, resets both bars, enters IDLE. No analytics update.

### Enter Idle (`_enterIdleState`)

- Stops timer loop.
- Sets `timer-state = 'idle'` in GSettings.
- Sets icon to default `pomodoro-indicator-icon`.
- Both bars can enter idle breathing pulse or hide.

### Daily Reset (`_checkDailyReset`)

- Compares `last-session-date` to today's date string (`YYYY-MM-DD`).
- If different day: resets `sessions-completed`, `total-focus-minutes`, and `task-stats` to zero/empty.
- Preserves `all-time-focus-minutes` and `analytics-history`.

---

## 10. Task Management

### Data Model

Tasks are stored in the `tasks` GSettings key as a JSON array. Each task:

```javascript
{
    text: "Task name",
    category: "Category name",   // Optional, defaults to "General"
    done: false,                 // Completion status
    id: Date.now(),             // Unique ID (timestamp)
    pomodorosSpent: 0,          // Auto-incremented when work sessions complete in this category
    completedAt: null           // ISO timestamp when marked done, null if active
}
```

### Task Operations (all in prefs.js, Tasks page)

| Operation | UI Element | Effect |
|---|---|---|
| **Add task** | Task Name entry + Add button | Creates task object, appends to array, saves to `tasks` key |
| **Mark done** | ✓ button on active task row | Sets `done = true`, `completedAt = new Date().toISOString()`, saves |
| **Undo done** | ↩ button on completed task row | Sets `done = false`, `completedAt = null`, saves |
| **Delete** | 🗑 button on any task row | Removes from array. If task was active (not done), increments `deleted-incomplete-count`. Shows confirmation dialog warning about completion rate impact. |

### Automatic Pomodoro Tracking

In `extension.js`, when a work session completes (`_onTimerComplete`):
- Reads `timer-category` to get the current focus category.
- Iterates all tasks in the `tasks` array.
- For each active (not done) task whose `category` matches: increments `pomodorosSpent` by 1.
- Saves back to GSettings.

### Category-Based Focus

Categories connect tasks to timer sessions:
1. User creates tasks with a category (e.g., "Work", "Study", "Personal").
2. Before starting a focus session, user selects a category from the dropdown on the Home page.
3. When the session completes, all active tasks in that category get credit for the pomodoro.
4. The analytics pie chart shows time breakdown per category.

---

## 11. Analytics

### Data Tracked

| Metric | GSettings Key | Scope | Reset |
|---|---|---|---|
| Sessions completed today | `sessions-completed` | Daily | Daily auto-reset or manual |
| Focus minutes today | `total-focus-minutes` | Daily | Daily auto-reset or manual |
| All-time focus minutes | `all-time-focus-minutes` | Lifetime | Manual only |
| Category breakdown today | `task-stats` | Daily | Daily auto-reset or manual |
| 30-day history | `analytics-history` | 30 days rolling | Manual only |
| Best streak (consecutive days) | `best-streak` | Lifetime | Manual only |
| Deleted incomplete tasks | `deleted-incomplete-count` | Lifetime | Manual only |

### `analytics-history` Format

JSON array, max 30 entries:
```javascript
[
    {
        date: "2026-05-23",
        sessions: 4,
        focusMinutes: 100,
        categories: { "Work": 75, "Study": 25 },
        tasksCompleted: 2
    },
    // ... up to 30 entries
]
```

### Streak Calculation

Computed from `analytics-history`: counts consecutive days (going backwards from today) where `sessions > 0`. If today has 0 sessions and a history exists, the streak counts consecutive days before today.

`best-streak` is persisted and updated when the current streak exceeds it.

### Analytics Dashboard (prefs.js, Page 4)

- **Today's Focus group**: Pomodoros completed, total focus time, average session length (computed: `total-focus-minutes / sessions-completed`), all-time focus minutes.
- **Streak group**: Current streak, best streak.
- **Pie Chart**: 220×220 px `Gtk.DrawingArea`, donut chart of time per category. Each slice is a different color. Hover shows tooltip with category name and minutes. Clicking a bar in the 7-day chart filters the pie to that specific day's data.
- **Bar Chart**: 396×190 px `Gtk.DrawingArea`, shows daily focus minutes for the last 7 days. Today's bar is highlighted in pink. Selected bar (when clicked) turns gold and updates the pie chart.
- **Task Completion group**: Total tasks, completed, active, completion rate percentage (computed as `completed / (completed + active + deleted_incomplete)`).
- **Reset buttons**: "Reset Today's Stats" and "Reset All History" (destructive action buttons).

---

## 12. Motivational Quotes

### Source (`quotes.js`)

Exports an array `MOTIVATIONAL_QUOTES` with **52 quotes** from 5 categories:
- **Elon Musk** (10 quotes)
- **Naval Ravikant** (10 quotes)
- **Swami Vivekananda** (10 quotes)
- **Bhagavad Gita** (10 quotes)
- **General** (2 quotes — Zeigarnik effect & action-precedes-motivation)

Each quote is `{ text, author }`.

### Two Functions

1. **`getRandomQuote()`**: Fully random using `Math.random()`.
2. **`getTimedQuote(intervalHours = 8)`**: Returns a quote that stays the same within a time block (default 8 hours). Uses a simple LCG pseudo-random generator seeded by `Math.floor(Date.now() / msPerInterval)`, so the quote is deterministic within the same period.

### Motivation Messages in extension.js

`extension.js` also has a separate `MOTIVATIONS` constant with categorized messages:

| Category | When Used |
|---|---|
| `idle_zero` | No sessions completed yet (e.g., "You haven't started yet") |
| `idle_some` | Some sessions completed, currently idle |
| `work_early` | Early phase of work session |
| `work_mid` | Middle phase of work session |
| `work_late` | Late phase of work session |
| `break_msg` | During breaks |
| `streak` | Contains `{n}` placeholder — e.g., "{n} pomodoros done — keep the streak alive!" |

`getMotivation(category, n)` picks a random message and replaces `{n}`. Currently only the `streak` category is used in `extension.js` (in the work-complete notification body).

### Display

The motivation text appears below the timer orb on the Home page and updates based on the current timer state and progress phase.

---

## 13. Notifications & Sounds

### Notifications

| Event | Title | Body |
|---|---|---|
| Work session completes | `"Pomodoro Complete!"` | Session count + random streak motivation |
| Break completes | `"⏰ Break Over!"` | `"Time to get back to work. Stay focused!"` |

- Uses `Main.notify(title, body)` — standard GNOME Shell desktop notification.
- Gated by `show-notifications` boolean setting.

### Sounds

| Event | Sound |
|---|---|
| Work session completes | `global.display.get_sound_player().play_from_theme('complete', 'Pomodoro Complete', null)` |
| Break completes | `global.display.get_sound_player().play_from_theme('complete', 'Break Over', null)` |

- Uses GNOME's built-in sound theme system, playing the `'complete'` sound event.
- Gated by `play-sound-notifications` boolean setting.

---

## 14. Themes

### 9 Theme Presets

Each theme changes the color palette across both the top panel bars and the preferences window. Selecting a theme overwrites 3 color keys (`work-bar-color`, `break-bar-color`, `idle-bar-color`) to the theme's preset values.

| Theme | ID | Visual Character |
|---|---|---|
| Default Glass | `default` | Blue/cyan, clean and modern |
| Black Pink | `black-pink` | Hot pink gradients on dark |
| Aurora | `aurora` | Teal/cyan with purple accents |
| Emerald | `emerald` | Rich green tones |
| Amber | `amber` | Gold/yellow warmth |
| Violet | `violet` | Purple/lavender |
| Ocean | `ocean` | Sky blue hues |
| Graphite | `graphite` | Neutral gray, subtle |
| Custom | `custom` | User picks all colors manually |

### How Themes Apply

1. `theme-name` setting is read by both `extension.js` and `prefs.js`.
2. `extension.js` calls `_applyThemeClass()` which adds a CSS class like `.theme-aurora` to the top panel elements.
3. `prefs.js` applies a CSS class to the preferences window, and overwrites color settings when a preset is selected.
4. The stylesheet has per-theme CSS blocks that modify `popup-menu-content`, `timer-area`, `timer-circle`, `bar-outer`, fill colors, stats/task cards, and hover states.

### Custom Theme

When `'custom'` is selected, color keys are not overwritten — the user's manual color picker selections are preserved.

---

## 15. IPC: How the Two Processes Communicate

Since `extension.js` and `prefs.js` run in separate processes, they communicate through GSettings keys:

### Prefs → Extension (Commands)

| Key | Type | Values | Purpose |
|---|---|---|---|
| `timer-command` | string | `'start-work'`, `'start-short-break'`, `'start-long-break'`, `'pause'`, `'resume'`, `'skip'`, `'reset'`, `'idle'` | One-shot commands. Extension reads, executes, then clears the key. |
| `timer-category` | string | Any category name | Category for the current/next focus session |

**Flow**: User clicks "Start Focus" in prefs → prefs writes `'start-work'` to `timer-command` → `extension.js` detects the change via `_onTimerCommand()` handler → executes `_startWork()` → clears `timer-command` to `''`.

### Extension → Prefs (State Sync)

| Key | Type | Purpose |
|---|---|---|
| `timer-state` | string | Current state: `'idle'`, `'work'`, `'short_break'`, `'long_break'` |
| `timer-is-paused` | boolean | Whether timer is paused |
| `timer-start-time` | double | `GLib.get_monotonic_time()` when timer last started/resumed |
| `timer-elapsed-accumulated` | double | Seconds elapsed before the current running segment (used for pause/resume) |
| `timer-duration-secs` | int | Total duration of current timer in seconds |

**Flow**: Extension updates these keys on every state change → Prefs window subscribes to `changed` signals on these keys → Prefs computes remaining time and updates the timer display.

### Bidirectional

| Key | Type | Both Read & Write |
|---|---|---|
| `timer-is-paused` | boolean | Both can set this |
| `tasks` | string (JSON) | Prefs manages task CRUD; extension updates `pomodorosSpent` |
| `sessions-completed` | int | Extension increments; prefs displays and can reset |
| `total-focus-minutes` | int | Extension increments; prefs displays and can reset |

---

## 16. Complete GSettings Schema (30 Keys)

**Schema ID**: `org.gnome.shell.extensions.pomodoro-timer`
**Schema path**: `/org/gnome/shell/extensions/pomodoro-timer/`

### User-Configurable Settings (21 keys)

| # | Key | Type | Default | Category | Description |
|---|---|---|---|---|---|
| 1 | `work-duration` | int | 25 | Timer | Work session minutes (1–60) |
| 2 | `short-break-duration` | int | 5 | Timer | Short break minutes (1–30) |
| 3 | `long-break-duration` | int | 15 | Timer | Long break minutes (1–60) |
| 4 | `long-break-interval` | int | 4 | Timer | Work sessions before long break (2–8) |
| 5 | `strict-mode` | bool | true | Behavior | Hide pause/skip during work |
| 6 | `auto-start-work` | bool | true | Behavior | Auto-start work after break |
| 7 | `auto-start-breaks` | bool | true | Behavior | Auto-start break after work |
| 8 | `show-notifications` | bool | true | Behavior | Desktop notifications |
| 9 | `play-sound-notifications` | bool | true | Behavior | Sound on completion |
| 10 | `bar-width` | int | 140 | Appearance | Bar width in px (60–300) |
| 11 | `bar-height` | int | 3 | Appearance | Bar height in px (2–8) |
| 12 | `bar-radius` | int | 99 | Appearance | Corner radius (0–99) |
| 13 | `work-bar-color` | string | `#00d4ff` | Appearance | Work bar color (hex) |
| 14 | `break-bar-color` | string | `#00ff66` | Appearance | Break bar color (hex) |
| 15 | `idle-bar-color` | string | `#e6b800` | Appearance | Idle pulse color (hex) |
| 16 | `color-saturation` | double | 1.0 | Appearance | Saturation multiplier (0.0–2.0) |
| 17 | `glow-intensity` | int | 3 | Appearance | Glow spread in px (0–20) |
| 18 | `icon-style` | string | `heart-outline` | Appearance | Icon style (heart-outline, heart-solid, ring, flame). Not exposed in prefs UI. |
| 19 | `theme-name` | string | `default` | Theme | Active theme preset |
| 20 | `heart-color` | string | `#999999` | Theme | Panel heart base color |
| 21 | `heart-outline-color` | string | `#ff0000` | Theme | Panel heart progress color |

### Internal State / Analytics Keys (9 keys, not directly user-editable)

| # | Key | Type | Default | Purpose |
|---|---|---|---|---|
| 22 | `sessions-completed` | int | 0 | Pomodoros completed today |
| 23 | `total-focus-minutes` | int | 0 | Focus minutes today |
| 24 | `all-time-focus-minutes` | int | 0 | Lifetime focus minutes |
| 25 | `last-session-date` | string | `''` | Date for daily reset (YYYY-MM-DD) |
| 26 | `tasks` | string | `'[]'` | JSON task array |
| 27 | `deleted-incomplete-count` | int | 0 | Deleted active tasks counter |
| 28 | `task-stats` | string | `'{}'` | Per-category minutes today (JSON) |
| 29 | `analytics-history` | string | `'[]'` | 30-day history array (JSON) |
| 30 | `best-streak` | int | 0 | Best consecutive days streak |

### IPC Keys (7 keys, used for inter-process communication)

| # | Key | Type | Default | Direction |
|---|---|---|---|---|
| 31 | `timer-command` | string | `''` | Prefs → Extension |
| 32 | `timer-category` | string | `General` | Prefs → Extension |
| 33 | `timer-state` | string | `idle` | Extension → Prefs |
| 34 | `timer-start-time` | double | 0.0 | Extension → Prefs |
| 35 | `timer-elapsed-accumulated` | double | 0.0 | Extension → Prefs |
| 36 | `timer-duration-secs` | int | 0 | Extension → Prefs |
| 37 | `timer-is-paused` | bool | false | Bidirectional |
| 38 | `bday-surprise` | bool | false | Birthday Easter egg trigger |

---

## 17. CSS Classes Reference

### Progress Bars (Top Panel)

| Class | Element |
|---|---|
| `.pomodoro-bar-container` | Outer wrapper for bar |
| `.pomodoro-work-bar-container` / `.pomodoro-break-bar-container` | Position-specific margins |
| `.pomodoro-bar-hidden` / `.pomodoro-bar-visible` | Show/hide with opacity |
| `.pomodoro-bar-outer` | Glass capsule (rounded pill, translucent borders, specular highlights) |
| `.pomodoro-bar-track` | Dark inset background track |
| `.pomodoro-bar-fill` | Progress fill element |
| `.pomodoro-bar-shine` | Top glass overlay |
| `.pomodoro-work-fill` | Work fill (white→cyan gradient) |
| `.pomodoro-break-fill` | Break fill (white→green gradient) |
| `.pomodoro-work-fill-urgent` | Urgent fill at >80% (white→amber gradient) |
| `.pomodoro-complete-fill` | Completion flash (bright white glow) |
| `.pomodoro-bar-time-label` | Countdown text (monospace, 10px) |
| `.pomodoro-bar-time-active` | Active time label (brighter) |
| `.pomodoro-bar-state-urgent` | Parent state for urgent visual |
| `.pomodoro-bar-state-complete` | Parent state for complete visual |

### Panel Indicator

| Class | Element |
|---|---|
| `.pomodoro-indicator` | PanelMenu.Button wrapper |
| `.pomodoro-indicator-icon` | Default icon (dimmed) |
| `.pomodoro-indicator-icon-active` | Work state icon (bright + glow) |
| `.pomodoro-indicator-icon-break` | Break state icon |

### Timer Orb (Prefs Home Page — CSS in prefs.js)

| Class | Element |
|---|---|
| `.pomodoro-timer-area` | Glass card containing timer |
| `.pomodoro-timer-circle` | 232–240px circular orb |
| `.pomodoro-timer-state-label` | "FOCUS" / "BREAK" text |
| `.pomodoro-timer-digits` | Large countdown digits (60–64px) |
| `.pomodoro-timer-play-btn` | Play/Pause button |
| `.pomodoro-timer-play-icon` | Icon inside play button |
| `.pomodoro-ring-dot` | Glowing bead on session ring (14px) |
| `.pomodoro-motivation-text` | Motivational quote below timer |

### Stats

| Class | Element |
|---|---|
| `.pomodoro-stats-card` | Glass pane for statistics |
| `.pomodoro-stats-icon` | Icon in stats section |
| `.pomodoro-stats-label-primary` | "Sessions" / "Minutes" label |
| `.pomodoro-stats-value` | Numeric stat value |
| `.pomodoro-stats-dot` | Separator dot |

### Tasks

| Class | Element |
|---|---|
| `.pomodoro-tasks-header` | "TASKS" section header |
| `.pomodoro-task-box` | Glass container for task list |
| `.pomodoro-task-row` | Individual task row (hover glow) |
| `.pomodoro-task-circle` / `.pomodoro-task-circle-done` | Circular checkbox |
| `.pomodoro-task-text` / `.pomodoro-task-text-done` | Task label (done = strikethrough) |
| `.pomodoro-task-entry` | Text input for adding tasks |
| `.pomodoro-task-add-btn` | Add task button |
| `.pomodoro-task-delete` | Delete button (hover turns red) |

### Controls

| Class | Element |
|---|---|
| `.pomodoro-start-session-btn` | Large CTA "Start Focus Session" |
| `.pomodoro-start-session-icon` | Icon inside start button |
| `.pomodoro-start-session-label` | Button text |
| `.pomodoro-start-session-chevron` | Right arrow chevron |

### Bottom Bar

| Class | Element |
|---|---|
| `.pomodoro-bottom-bar` | Container for settings + power |
| `.pomodoro-settings-btn` | Settings button |
| `.pomodoro-power-button` | Circular toggle |
| `.pomodoro-power-on` / `.pomodoro-power-off` | Toggle states |

### Theme Classes (applied to root containers)

`.theme-default`, `.theme-black-pink`, `.theme-aurora`, `.theme-emerald`, `.theme-amber`, `.theme-violet`, `.theme-ocean`, `.theme-graphite`, `.theme-custom`

---

## 18. Icons (27 SVGs)

All in the `icons/` directory, following GNOME's `-symbolic` naming convention:

| Icon | Purpose |
|---|---|
| `polindora-logo.svg` | Main app logo (panel indicator, 16px) |
| `polindora-active-symbolic.svg` | Active state indicator |
| `polindora-analytics-symbolic.svg` | Analytics tab |
| `polindora-appearance-symbolic.svg` | Appearance/theme settings tab |
| `polindora-avg-symbolic.svg` | Average statistics |
| `polindora-behavior-symbolic.svg` | Behavior settings tab |
| `polindora-bullet-symbolic.svg` | Bullet point / list item |
| `polindora-complete-symbolic.svg` | Completion checkmark |
| `polindora-delete-symbolic.svg` | Delete/trash action |
| `polindora-home-symbolic.svg` | Home tab |
| `polindora-hourglass-symbolic.svg` | Timer/waiting |
| `polindora-inbox-symbolic.svg` | Inbox/tasks |
| `polindora-list-symbolic.svg` | List view |
| `polindora-pause-symbolic.svg` | Pause action |
| `polindora-pending-symbolic.svg` | Pending/in-progress |
| `polindora-percent-symbolic.svg` | Percentage/completion rate |
| `polindora-play-symbolic.svg` | Play/start action |
| `polindora-pomodoro-symbolic.svg` | Pomodoro/tomato |
| `polindora-reset-symbolic.svg` | Reset action |
| `polindora-rocket-symbolic.svg` | Launch/start session |
| `polindora-skip-symbolic.svg` | Skip action |
| `polindora-star-symbolic.svg` | Star/favorite |
| `polindora-streak-symbolic.svg` | Streak counter |
| `polindora-tasks-symbolic.svg` | Tasks tab |
| `polindora-timer-symbolic.svg` | Timer settings tab |
| `polindora-trophy-symbolic.svg` | Achievement/trophy |
| `polindora-undo-symbolic.svg` | Undo action |

---

## 19. Installation

### `install.sh`

1. **Safety**: Refuses to run as root/sudo (GNOME extensions must be user-local).
2. **Validates** `metadata.json` exists and extracts UUID via grep.
3. **Checks required tools**: `glib-compile-schemas`, `gnome-extensions`, `gsettings`, `cp`, `mkdir`.
4. **Enables user extensions**: `gsettings set org.gnome.shell disable-user-extensions false`.
5. **Installs** to `~/.local/share/gnome-shell/extensions/polindora@yosantosh.github.io/`:
   - Copies: `extension.js`, `prefs.js`, `metadata.json`, `stylesheet.css`, `quotes.js`
   - Copies directories: `schemas/`, `icons/`
6. **Compiles GSettings schema**: `glib-compile-schemas schemas/`.
7. **Enables**: `gnome-extensions enable polindora@yosantosh.github.io`.
8. **Post-install**: User must restart GNOME Shell (Wayland: log out/in; X11: Alt+F2 → `r`).

### `uninstall.sh`

Reverses installation with prompts to keep or wipe user data.

---

## 20. UI Element → Function Reference

### Top Panel (extension.js)

| UI Element | User Action | Function Called | Effect |
|---|---|---|---|
| Panel icon (♥ logo) | Left click | `_openPreferences()` (via hijacked `menu.toggle()`) | Opens preferences window |
| Work progress bar | Single click | `_onBarClicked()` | Pause or resume timer |
| Break progress bar | Single click | `_onBarClicked()` | Pause or resume timer |
| Either bar | Double click (<300ms) | `_onBarClicked()` → `_openPreferences()` | Opens preferences window |

### Preferences Home Page (prefs.js → IPC → extension.js)

| UI Element | User Action | IPC Command Written | extension.js Handler |
|---|---|---|---|
| Start Focus button | Click | `'start-work'` | `_startWork()` |
| Pause button | Click | `'pause'` | `_togglePause()` |
| Resume button | Click | `'resume'` | `_togglePause()` |
| Skip button | Click | `'skip'` | `_skip()` |
| Reset button | Click | `'reset'` | `_reset()` |
| Category dropdown | Select | Writes to `timer-category` | Read at `_onTimerComplete()` |
| Settings gear | Click | — | Opens settings sub-window (prefs-only) |
| Task "Mark done" | Click | Writes to `tasks` JSON | (prefs-only) |

### Preferences Settings Pages (prefs.js only)

| Page | UI Element | User Action | Effect |
|---|---|---|---|
| Timer | SpinRow (any duration) | Change value | Writes to GSettings, picked up next session start |
| Behavior | Strict Mode switch | Toggle | Writes `strict-mode`, prefs hides/shows pause/skip buttons |
| Behavior | Auto-start switches | Toggle | Writes `auto-start-work` / `auto-start-breaks` |
| Behavior | Notification switches | Toggle | Writes `show-notifications` / `play-sound-notifications` |
| Appearance | Bar dimension SpinRows | Change value | Writes to GSettings, extension reads on `_onSettingChanged()` and calls `_applyDimensionsAndAppearance()` |
| Appearance | Color pickers | Pick color | Writes hex string, extension calls `_applyWorkColor()` / `_applyBreakColor()` |
| Appearance | Reset Colors button | Click | Resets 5 color keys to Default Glass preset values |
| Analytics | Reset Today's Stats | Click | Zeros `sessions-completed`, `total-focus-minutes`, `task-stats` |
| Analytics | Reset All History | Click | Zeros all analytics keys |
| Tasks | Add Task button | Click | Appends to `tasks` JSON |
| Tasks | Mark done / Undo / Delete | Click | Modifies `tasks` JSON |
| Theme | Theme dropdown | Select | Writes `theme-name`, overwrites color keys to preset, extension applies CSS class |
| Theme | Custom color pickers | Pick color | Writes to color keys (only when theme = `'custom'`) |

---

## End

This document covers every feature, every UI element, every setting, every function, and how they all connect. With this, you can rebuild the entire Polindora App from scratch.
