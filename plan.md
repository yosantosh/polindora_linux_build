# Polindora Linux Build Plan

## 0. Implementation Progress

This section is the running build tracker. Keep it updated whenever a task is
completed, blocked, or moved into active work.

### Current Phase: Foundation Vertical Slice

- [x] Create root `AGENTS.md` with Codex and Antigravity A2A agent rules.
- [x] Create initial Rust project metadata in `Cargo.toml`.
- [x] Add Debian packaging metadata and desktop launcher paths.
- [x] Create required root ownership directories: `src/`, `qml/`,
  `qml/components/`, `shaders/`, `assets/`, `migrations/`, and `packaging/`.
- [x] Add Rust timer state machine foundation in `src/timer_engine.rs`.
- [x] Add Rust task persistence API foundation in `src/task_store.rs`.
- [x] Add Rust analytics summary foundation in `src/analytics.rs`.
- [x] Add initial bridge-facing state snapshot in `src/bridge.rs`.
- [x] Add initial SQLite schema in `migrations/0001_initial.sql`.
- [x] Add first QML home shell matching the target home-page direction.
- [x] Add reusable QML glass panel, button, progress ring, and timer orb
  components.
- [x] Add initial GLSL shader files for future glass/orb rendering.
- [x] Add `resources.qrc` entries for QML and shader resources.
- [x] Replace the temporary Rust bridge facade with real CXX-Qt QObject
  properties, signals, and slots.
- [x] Wire the Rust application entry point to load the QML engine.
- [x] Implement Timer settings page.
- [x] Implement Behavior settings page.
- [x] Implement Appearance settings page.
- [x] Implement Analytics page backed by real session history.
- [x] Implement Tasks page UI and controller-backed task interactions.
- [x] Add SQLite persistence adapter for task storage.
- [ ] Wire Tasks page controller interactions directly to SQLite persistence.
- [x] Implement Theme page and runtime theme switching.
- [ ] Add packaging verification with `cargo build --release` and `cargo deb`.

### Active Build Strategy

- Build source surface first, then run full `cargo test` / `cargo build` after
  the primary app screens and bridge contracts are complete.
- Maintain standalone custom core tests for pure Rust modules so timer and
  analytics behavior can be checked without invoking the CXX-Qt build step.
- Use full Cargo verification as the integration gate once the QML/Rust bridge
  stabilizes.

### Verification Log

- Passed: `xmllint --noout resources.qrc`.
- Passed: Rust/Cargo and Qt 6 development tooling were installed through apt.
- Passed: static read-back of CXX-Qt bridge declarations, QML settings shell,
  and resource registrations.
- Passed: static read-back of backend-backed Timer and Behavior settings
  controls.
- Passed: static read-back of backend-backed Appearance settings controls.
- Passed: static read-back of Analytics tab bindings to controller-owned
  session history and Rust analytics summaries.
- Passed: static read-back of Tasks tab controller bindings and resource
  registration.
- Passed: standalone custom core tests with `bash scripts/run_custom_core_tests.sh`
  after fixing countdown display rounding in `src/timer_engine.rs`.
- Passed: static read-back of Theme tab controller bindings and resource
  registration.
- Passed: standalone custom core/theme tests with `bash scripts/run_custom_core_tests.sh`
  after adding `tests/custom/core_logic_tests.rs` and
  `tests/custom/theme_tests.rs`.
- Deferred: full `cargo test` and `cargo build` until the app source surface is
  complete enough for an integration build.

## 1. Purpose

This document is the build plan for the target Polindora Linux app using the available project references:

- `Targets/Target_ui/target_home_page.png`
- `Targets/Target_ui/timer_tab.png`
- `Targets/Target_ui/behavioir_tab.png`
- `Targets/Target_ui/appearance_tab.png`
- `Targets/Target_ui/analytics_tab.png`
- `Targets/Target_ui/tasks_tab.png`
- `Targets/Target_ui/Theme_tab.png`
- `Targets/Target_functionalityties/polindora_everything.md`
- `Polindora_linux_build_Technical Stack & Architecture Document.md`

The goal is to build a premium native Linux Pomodoro productivity application with QML + QtQuick for the interface and Rust for the application logic. The older GNOME Shell extension documentation is used as product and behavior reference material, not as the target implementation architecture.

## 2. Source Interpretation And Architecture Decision

### 2.1 Primary Product Decision

The final product is a native QML + Rust Linux desktop application.

Reasoning:

- The technical stack document explicitly chooses QML + QtQuick for the frontend.
- The technical stack document explicitly chooses Rust for backend logic.
- The requested build is in a `polindora_linux_build` workspace.
- The target UI screenshots show a full app surface with highly custom glassmorphism that is better matched by QML/QtQuick than by stock GTK/libadwaita widgets.
- The GNOME Shell extension material represents the previous Polindora implementation and should inform behavior, settings, workflows, and feature completeness.

### 2.2 Role Of The GNOME Extension Documentation

The functionality document describes the older Polindora GNOME Shell extension, including top-panel progress bars, a panel icon, GSettings IPC, and a GTK4 preferences window. For this native app build, that document should be treated as a behavioral specification and legacy reference.

Resolution:

- Build the core application as a QML + Rust desktop app.
- Port the useful product behavior into Rust services and QML screens.
- Recreate the target UI using QML components, not GTK4/libadwaita widgets.
- Treat GNOME top-panel bars as an optional Linux desktop integration layer, not the core app.
- Keep any panel integration optional at packaging time if needed, but design the core state model so an extension can consume the same timer state without owning timer logic.

### 2.3 Persistence Decision

The previous GNOME extension used GSettings as its main persistence and IPC mechanism because GNOME Shell extensions fit that model. The native QML + Rust app should use SQLite + SQLx as its canonical persistence layer.

Resolution:

- Use SQLite + SQLx for the native app's canonical data: tasks, sessions, analytics, settings, themes, and history.
- Do not model the native app around GTK4/GSettings preferences architecture.
- Use GSettings only if a future GNOME Shell panel integration needs compatibility with GNOME settings/watchers.
- Prefer D-Bus for live app-to-extension state if panel integration is built.

### 2.4 UI Screenshot Additions Override The Older GNOME UI

The UI screenshots add features that are not fully present in `polindora_everything.md`. These should be included in the build plan:

- Settings search button.
- Full bottom tab navigation across settings pages.
- Large custom stepper controls instead of simple spin rows.
- Rich analytics overview cards with trend deltas.
- Daily / weekly / monthly analytics segmented control.
- Best focus time chart.
- Session history list.
- Task priorities.
- Task due dates.
- Task category filtering.
- Task sort order.
- Theme cards with visual previews.
- Auto theme switching.
- Accent color swatches, including custom/rainbow accent.
- Home-page summary rows with chevrons.

### 2.5 Known Visual Conflict

The Timer screenshot displays the Timer page content but appears to highlight the Theme tab in the bottom navigation. This should be treated as a screenshot inconsistency, not desired behavior.

Implementation rule:

- The active bottom tab must always match the visible page.

## 3. Success Criteria

The build is successful when:

1. The app launches as a native Linux desktop application.
2. The home page visually matches `target_home_page.png` at the intended window size.
3. The settings window contains Timer, Behavior, Appearance, Analytics, Tasks, and Theme tabs.
4. Every UI control shown in the screenshots has an implemented state model and backend action.
5. Timer state transitions work correctly: idle, work, short break, long break, pause, resume, skip, reset.
6. Strict mode hides pause and skip controls during work sessions.
7. Tasks can be created, completed, filtered, sorted, prioritized, and tracked by category.
8. Analytics are computed from real session history, not hardcoded demo data.
9. Themes and accent colors update the app visually.
10. Settings persist across app restarts.
11. `.deb` packaging works.
12. Optional GNOME panel integration can read timer state and render panel progress bars if included.

## 4. Product Architecture

### 4.1 High-Level Architecture

```text
QML / QtQuick UI
    |
    | CXX-Qt properties, signals, slots
    v
Rust App Bridge
    |
    +-- Timer Engine
    +-- Task Service
    +-- Analytics Service
    +-- Theme Service
    +-- Settings Service
    +-- Notification Service
    +-- Persistence Service
    |
    v
SQLite Database

Optional GNOME Integration
    |
    +-- GNOME Shell extension panel bars
    +-- GSettings or D-Bus state bridge
```

### 4.2 Core Design Principle

The timer engine and app state must live in Rust. QML should render state and send user intents. QML should not own timer rules, analytics calculations, or persistence rules.

### 4.3 Frontend Responsibilities

QML owns:

- Window structure.
- Layout.
- Navigation.
- Visual styling.
- Glassmorphism components.
- Animated timer orb.
- Settings pages.
- Charts.
- Forms.
- Hover, press, and transition states.
- Theme application.

QML does not own:

- Timer completion rules.
- Long-break interval decisions.
- Daily reset logic.
- Task analytics.
- Session persistence.
- Notification business rules.

### 4.4 Rust Responsibilities

Rust owns:

- Timer state machine.
- Accurate monotonic timing.
- Pause/resume elapsed-time accounting.
- Task CRUD.
- Task completion metrics.
- Session history.
- Analytics aggregation.
- Theme preset definitions.
- Settings validation.
- SQLite migrations.
- Desktop notifications.
- Optional GNOME integration state publishing.

### 4.5 Optional GNOME Panel Integration

GNOME Shell extensions cannot be written directly in QML/Rust because they run inside GNOME Shell's JavaScript runtime. Therefore the top-panel progress bars must be implemented as a small JS extension if the panel feature is required.

The panel extension should:

- Render the work bar to the right of the GNOME clock.
- Render the break bar to the left of the GNOME clock.
- Render the Polindora panel icon in the status area.
- Open the native Polindora app when clicked.
- Read live timer state from GSettings or D-Bus.
- Avoid duplicating business logic.

The native app remains the source of truth.

## 5. Recommended Repository Structure

```text
polindora_linux_build/
├── Cargo.toml
├── build.rs
├── plan.md
├── migrations/
│   ├── 0001_initial.sql
│   ├── 0002_task_metadata.sql
│   └── 0003_session_history.sql
├── src/
│   ├── main.rs
│   ├── bridge/
│   │   ├── mod.rs
│   │   ├── app_controller.rs
│   │   ├── timer_bridge.rs
│   │   ├── task_bridge.rs
│   │   ├── analytics_bridge.rs
│   │   └── theme_bridge.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── timer.rs
│   │   ├── task.rs
│   │   ├── session.rs
│   │   ├── analytics.rs
│   │   ├── settings.rs
│   │   └── theme.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── timer_service.rs
│   │   ├── task_service.rs
│   │   ├── analytics_service.rs
│   │   ├── settings_service.rs
│   │   ├── notification_service.rs
│   │   └── integration_service.rs
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── db.rs
│   │   ├── task_repository.rs
│   │   ├── session_repository.rs
│   │   └── settings_repository.rs
│   └── util/
│       ├── time.rs
│       └── validation.rs
├── qml/
│   ├── Main.qml
│   ├── AppWindow.qml
│   ├── HomePage.qml
│   ├── SettingsWindow.qml
│   ├── pages/
│   │   ├── TimerPage.qml
│   │   ├── BehaviorPage.qml
│   │   ├── AppearancePage.qml
│   │   ├── AnalyticsPage.qml
│   │   ├── TasksPage.qml
│   │   └── ThemePage.qml
│   ├── components/
│   │   ├── GlassPanel.qml
│   │   ├── GlassButton.qml
│   │   ├── IconButton.qml
│   │   ├── SearchButton.qml
│   │   ├── BottomTabBar.qml
│   │   ├── NavTab.qml
│   │   ├── TimerOrb.qml
│   │   ├── ProgressArc.qml
│   │   ├── SessionRing.qml
│   │   ├── StepperRow.qml
│   │   ├── ToggleRow.qml
│   │   ├── SliderCard.qml
│   │   ├── ColorSwatch.qml
│   │   ├── ThemeCard.qml
│   │   ├── StatCard.qml
│   │   ├── TaskRow.qml
│   │   ├── CategoryDropdown.qml
│   │   ├── SegmentedControl.qml
│   │   ├── BarChart.qml
│   │   ├── DonutChart.qml
│   │   └── LineChart.qml
│   ├── effects/
│   │   ├── GlassMaterial.qml
│   │   ├── GlowLayer.qml
│   │   └── ShimmerLayer.qml
│   └── theme/
│       ├── ThemeManager.qml
│       ├── palettes.js
│       └── typography.js
├── assets/
│   ├── icons/
│   ├── fonts/
│   ├── theme-previews/
│   └── sounds/
├── shaders/
│   ├── glass.frag
│   ├── orb.frag
│   └── shimmer.frag
├── packaging/
│   ├── deb/
│   ├── polindora.desktop
│   └── icons/
└── gnome-extension/
    ├── metadata.json
    ├── extension.js
    ├── stylesheet.css
    └── schemas/
```

## 6. Runtime Modules

### 6.1 App Controller

The app controller is the top-level Rust object exposed to QML. It coordinates services and exposes read-only view state plus intent methods.

Responsibilities:

- Initialize database.
- Load settings.
- Start timer service.
- Expose current page state.
- Expose derived UI values.
- Publish notifications to QML.
- Publish integration state to GNOME if enabled.

Example QML-facing methods:

- `startWork(category: string)`
- `pauseTimer()`
- `resumeTimer()`
- `skipTimer()`
- `resetTimer()`
- `setDuration(kind: string, minutes: int)`
- `setStrictMode(enabled: bool)`
- `createTask(input: TaskInput)`
- `completeTask(id: string)`
- `undoTask(id: string)`
- `deleteTask(id: string)`
- `setTheme(themeId: string)`
- `setAccentColor(color: string)`

### 6.2 Timer Service

The timer service owns the timer state machine.

States:

- `Idle`
- `Work`
- `ShortBreak`
- `LongBreak`

Overlay state:

- `paused: bool`

Timer fields:

- `state`
- `duration_secs`
- `started_at_monotonic`
- `elapsed_accumulated_secs`
- `remaining_secs`
- `progress`
- `active_category`
- `completed_in_cycle`
- `strict_mode`

Timer commands:

- `StartWork`
- `StartShortBreak`
- `StartLongBreak`
- `Pause`
- `Resume`
- `Skip`
- `Reset`

Completion behavior:

- Work completion increments session counters.
- Work completion records a focus session.
- Work completion updates category analytics.
- Work completion increments pomodoro count on matching active tasks.
- Work completion decides short break vs long break.
- Break completion optionally starts the next work session.

### 6.3 Task Service

The task service owns task CRUD and task-derived metrics.

Task fields:

- `id`
- `title`
- `category`
- `status`
- `priority`
- `due_date`
- `created_at`
- `completed_at`
- `pomodoros_spent`

Task status:

- `active`
- `completed`

Task priority:

- `low`
- `medium`
- `high`

Required operations:

- Create task.
- Mark task done.
- Undo completion.
- Delete task.
- Filter active tasks by category.
- Sort tasks by newest, oldest, priority, due date.
- Compute total, completed, remaining, completion rate.

### 6.4 Analytics Service

Analytics must be derived from persisted session records. It should not depend on the UI.

Metrics:

- Sessions today.
- Focus minutes today.
- Focus minutes this week.
- Focus minutes this month.
- Completion rate.
- Current streak.
- Best streak.
- Category distribution.
- Daily focus chart.
- Weekly focus chart.
- Monthly focus chart.
- Best focus time window.
- Session history.
- Trends vs previous period.

Screenshot-required additions:

- Trend deltas like `+20% vs last week`.
- Timeframe dropdown such as `This Week`.
- Segmented chart modes: Daily, Weekly, Monthly.
- Best focus time range such as `10:00 AM - 12:00 PM`.
- Session history rows with type, timestamp, duration, status, and details action.

### 6.5 Theme Service

The theme service owns theme metadata and palette selection.

Theme fields:

- `id`
- `name`
- `description`
- `mode`
- `background`
- `surface`
- `surface_elevated`
- `border`
- `text_primary`
- `text_secondary`
- `accent`
- `accent_secondary`
- `success`
- `warning`
- `danger`
- `preview_asset`

Required themes from screenshots:

- `polindora-dark`
- `midnight-blue`
- `ocean-breeze`
- `sunset-glow`
- `aurora`
- `light-mode`

Required accent swatches:

- Violet
- Blue
- Cyan
- Green
- Amber
- Orange
- Pink
- Custom/rainbow

Compatibility mapping from the functionality document:

- Existing theme presets such as Default Glass, Black Pink, Emerald, Graphite, and Custom can be implemented as accent presets or future additional theme cards.

### 6.6 Settings Service

The settings service validates and persists app configuration.

Settings groups:

- Timer.
- Behavior.
- Appearance.
- Analytics preferences.
- Task preferences.
- Theme.
- Integration.

Important validations:

- Work duration: 1 to 60 minutes.
- Short break duration: 1 to 30 minutes.
- Long break duration: 1 to 60 minutes.
- Long break interval: 2 to 8 pomodoros.
- Bar width: 60 to 300 px.
- Bar height: use screenshot target 4 to 12 px unless compatibility requires legacy 2 to 8 px.
- Bar radius: 0 to 99.
- Color saturation: 0.0 to 2.0.
- Glow intensity: 0 to 20.

## 7. Database Design

### 7.1 `settings`

```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

Store settings as typed JSON values. Keep service-level validators so invalid values are not written.

### 7.2 `tasks`

```sql
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'General',
    status TEXT NOT NULL CHECK (status IN ('active', 'completed')),
    priority TEXT NOT NULL DEFAULT 'medium' CHECK (priority IN ('low', 'medium', 'high')),
    due_date TEXT,
    created_at TEXT NOT NULL,
    completed_at TEXT,
    pomodoros_spent INTEGER NOT NULL DEFAULT 0
);
```

### 7.3 `sessions`

```sql
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    session_type TEXT NOT NULL CHECK (session_type IN ('work', 'short_break', 'long_break')),
    category TEXT NOT NULL DEFAULT 'General',
    started_at TEXT NOT NULL,
    completed_at TEXT,
    duration_secs INTEGER NOT NULL,
    elapsed_secs INTEGER NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('completed', 'skipped', 'reset')),
    task_ids_json TEXT NOT NULL DEFAULT '[]'
);
```

### 7.4 `daily_stats`

This table can be derived from `sessions`, but caching it may simplify fast startup.

```sql
CREATE TABLE daily_stats (
    date TEXT PRIMARY KEY,
    sessions_completed INTEGER NOT NULL DEFAULT 0,
    focus_minutes INTEGER NOT NULL DEFAULT 0,
    tasks_completed INTEGER NOT NULL DEFAULT 0,
    categories_json TEXT NOT NULL DEFAULT '{}'
);
```

### 7.5 `deleted_task_stats`

```sql
CREATE TABLE deleted_task_stats (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    deleted_incomplete_count INTEGER NOT NULL DEFAULT 0
);
```

## 8. QML State Contract

### 8.1 Timer View State

Expose a `TimerViewState` object:

- `stateLabel`: `FOCUS`, `BREAK`, `LONG BREAK`, or `READY`.
- `timeText`: formatted `MM:SS`.
- `progress`: 0.0 to 1.0.
- `sessionRingSegments`: number of long-break interval segments.
- `sessionRingCompleted`: completed work sessions in the current cycle.
- `isRunning`.
- `isPaused`.
- `canPause`.
- `canSkip`.
- `canReset`.
- `primaryActionLabel`.
- `primaryActionIcon`.
- `activeCategory`.
- `motivationText`.

### 8.2 Home View State

Expose:

- Today pomodoros.
- Today focus minutes.
- Top active tasks.
- Available categories.
- Selected category.
- Current quote.

### 8.3 Settings View State

Expose:

- Timer settings.
- Behavior toggles.
- Appearance settings.
- Analytics summaries.
- Task metrics.
- Theme metadata.
- Search results.

## 9. Visual Design System

### 9.1 Overall Visual Direction

The UI should feel like dark saturated glass with precise neon accents. The screenshots use:

- Deep navy/black background.
- Subtle radial blue glow.
- Frosted translucent panels.
- Thin luminous borders.
- Blue, violet, cyan, and occasional green accents.
- Large rounded panels.
- Soft shadow depth.
- Crisp white primary text.
- Muted blue-gray secondary text.
- Icon circles with glow.

### 9.2 Layout Density

This app is a functional productivity tool, not a landing page. The UI can be visually rich, but every panel must serve an actual workflow.

Rules:

- No decorative cards that do not hold app state or controls.
- Keep controls large enough for comfortable desktop use.
- Avoid nested floating cards except repeated items and real tool panels.
- Maintain predictable scanning order.
- Keep all text inside its container at compact widths.

### 9.3 Window Shape

Home page target:

- Tall compact app window.
- Rounded outer border.
- Dark glass background.
- Header with centered `Polindora`.
- Close button top-right.
- Settings button floating right of timer orb.

Settings target:

- Tall compact settings window.
- Header with search button left, centered `Settings`, close button right.
- Bottom tab bar.
- Scrollable content area.

### 9.4 Typography

Use a modern sans-serif for UI labels and a monospaced or tabular numeric font for timer digits.

Suggested hierarchy:

- App title: 30 to 36 px, semibold.
- Settings title: 28 to 34 px, semibold.
- Section title: 22 to 28 px, semibold.
- Row title: 18 to 24 px, medium.
- Row description: 14 to 18 px, regular.
- Timer digits: 72 to 96 px, tabular numeric.
- Chart labels: 12 to 14 px.

Timer digits must use tabular numbers so the orb does not shift every second.

### 9.5 Core Components

Build these reusable QML components before assembling pages:

- `GlassPanel`
- `GlassButton`
- `IconButton`
- `BottomTabBar`
- `NavTab`
- `TimerOrb`
- `ProgressArc`
- `SessionRing`
- `StepperRow`
- `ToggleRow`
- `SliderCard`
- `ColorSwatch`
- `ThemeCard`
- `StatCard`
- `TaskRow`
- `CategoryDropdown`
- `SegmentedControl`
- `BarChart`
- `DonutChart`
- `LineChart`

## 10. Home Page Plan

### 10.1 Source

Primary source:

- `Targets/Target_ui/target_home_page.png`

Functional source:

- Preferences Home Page section in `polindora_everything.md`

### 10.2 Layout

Top to bottom:

1. Outer rounded glass app frame.
2. Header row.
3. Large central timer orb.
4. Motivation quote area.
5. Controls row.
6. Today summary section.
7. Top Tasks summary section.

### 10.3 Header

Elements:

- Centered app title: `Polindora`.
- Small decorative glow dots near the title.
- Close button at top-right.
- Thin horizontal divider below header.

Close behavior:

- If app is a normal desktop app, close hides or exits depending on tray setting.
- If GNOME integration is active, close should hide to background by default so the timer can continue.

### 10.4 Settings Button

The settings button is a circular glowing button on the right side of the orb area.

Behavior:

- Opens the settings view/window.
- Uses gear icon.
- Has hover glow.
- Has press scale animation.

### 10.5 Timer Orb

Visual:

- Large glass sphere.
- Inner dark radial gradient.
- Left-side white-blue specular highlight.
- Blue/violet lower glow.
- Outer progress arc.
- Progress bead at arc head.
- Heart icon above state text.
- State label with spaced letters.
- Large timer digits.

Implementation:

- `TimerOrb.qml` contains static orb visuals.
- `ProgressArc.qml` draws the timer progress.
- `SessionRing.qml` draws completed pomodoro segments.
- The progress arc should animate smoothly from previous progress to new progress every tick.
- The bead should move along the arc using trigonometric positioning.

States:

- Idle: display default work duration and `READY` or `FOCUS`.
- Work: display `FOCUS`.
- Short break: display `BREAK`.
- Long break: display `LONG BREAK`.
- Paused: show paused visual treatment without changing state label.

### 10.6 Motivation Quote

Visual:

- Quote mark icon at left.
- Quote text in italic or soft regular style.
- Author in accent blue/violet.

Behavior:

- Idle with no sessions: show start-focused message.
- Work early/mid/late: show progress-aware motivation.
- Break: show recovery-focused message.
- Completed sessions: can show streak motivation.
- Quote can refresh by timed interval, not every second.

### 10.7 Controls

Controls shown in screenshot:

- Category dropdown.
- Start button.
- Skip button.
- Reset button.

Functional additions:

- Pause and Resume must exist, but visibility depends on state and strict mode.

Rules:

- Idle: show category dropdown, Start, Reset disabled or low emphasis.
- Work running with strict mode on: hide Pause and Skip; keep Reset if spec allows reset during strict mode.
- Work running with strict mode off: show Pause and Skip.
- Paused: show Resume, Skip, Reset.
- Break: show Pause, Skip, Reset.

Category dropdown:

- Defaults to `General`.
- Populates from task categories plus `General`.
- Selection is persisted for the next work session.

### 10.8 Today Summary

Screenshot row:

- Section label: `Today`.
- Row icon: stopwatch.
- Primary text: `0 pomodoros`.
- Secondary text: `0m focused`.
- Chevron at right.

Behavior:

- Clicking opens Analytics tab.
- Values update live after work completion.

### 10.9 Top Tasks Summary

Screenshot row:

- Section label: `Top Tasks`.
- Row icon: clipboard.
- Empty state: `No active tasks`.
- Secondary: `Add tasks from the Tasks tab`.
- Chevron at right.

Behavior:

- Clicking opens Tasks tab.
- If tasks exist, show top two active tasks.
- Each row can include quick complete action if space allows.

## 11. Settings Shell Plan

### 11.1 Source

Primary settings UI sources:

- `timer_tab.png`
- `behavioir_tab.png`
- `appearance_tab.png`
- `analytics_tab.png`
- `tasks_tab.png`
- `Theme_tab.png`

### 11.2 Global Settings Layout

Every settings page uses:

- Header bar.
- Search button at top-left.
- Center title `Settings`.
- Close button at top-right.
- Page intro block with circular icon, page title, and subtitle.
- Scrollable content.
- Bottom tab bar.

### 11.3 Search

Search is present in screenshots but not described in the functionality document. Implement it.

Behavior:

- Clicking search opens an overlay or expands a search field.
- Search indexes settings labels, descriptions, tasks, and analytics labels.
- Results show page name and matching row.
- Selecting a result navigates to the relevant page and focuses/highlights the control.

Search examples:

- `strict` -> Behavior / Strict Mode.
- `duration` -> Timer / Work Duration, Short Break, Long Break.
- `color` -> Appearance / Colors and Theme / Accent.
- `priority` -> Tasks.
- `week` -> Analytics.

### 11.4 Bottom Tab Bar

Tabs:

- Timer.
- Behavior.
- Appearance.
- Analytics.
- Tasks.
- Theme.

Each tab has:

- Icon.
- Label.
- Active glow.
- Active underline/pill.
- Hover state.
- Press state.

Implementation rules:

- The active tab must match the current page.
- The bottom bar should remain fixed while page content scrolls.
- Labels must not overlap at compact widths.

## 12. Timer Settings Page Plan

### 12.1 Source

Primary source:

- `Targets/Target_ui/timer_tab.png`

### 12.2 Content

Section:

- Title: `Session Durations`.
- Subtitle: `Configure how long each session lasts`.
- Icon: stopwatch.

Rows:

- Work Duration.
- Short Break.
- Long Break.
- Long Break Interval.

### 12.3 Row Design

Each row is a large glass panel with:

- Circular icon at left.
- Title.
- Description.
- Current numeric value.
- Minus button.
- Plus button.

The first row in the screenshot has an active border. Use this style when:

- Row is focused by keyboard.
- Row was just changed.
- Row is selected from search.

### 12.4 Values

Work Duration:

- Default: 25.
- Unit: minutes per focus session.
- Range: 1 to 60.

Short Break:

- Default: 5.
- Unit: minutes for short breaks.
- Range: 1 to 30.

Long Break:

- Default: 15.
- Unit: minutes for long breaks.
- Range: 1 to 60.

Long Break Interval:

- Default: 4.
- Unit: pomodoros before a long break.
- Range: 2 to 8.

### 12.5 Interactions

- Minus decrements by 1.
- Plus increments by 1.
- Long press repeats safely.
- Values persist immediately.
- If a timer is currently running, duration changes apply to the next session, not the active countdown.

## 13. Behavior Settings Page Plan

### 13.1 Source

Primary source:

- `Targets/Target_ui/behavioir_tab.png`

### 13.2 Sections

Section 1:

- Title: `Focus Enforcement`.
- Subtitle: `Settings that keep you accountable`.
- Icon: shield.

Rows:

- Strict Mode.
- Auto-start Work.
- Auto-start Breaks.

Section 2:

- Title: `Notifications`.
- Subtitle: `Stay informed, stay on track`.
- Icon: bell.

Rows:

- Desktop Notifications.
- Sound Notifications.

### 13.3 Toggles

Use custom glowing toggle switches, not default platform switches.

States:

- On: blue/cyan track with white knob at right.
- Off: dark translucent track with muted knob at left.

Behavior:

- Toggle writes setting immediately.
- Toggle has animated knob movement.
- Toggle has hover glow.

### 13.4 Strict Mode

Strict Mode behavior:

- When enabled during work sessions, Pause and Skip controls are hidden.
- Break sessions still allow Pause and Skip.
- Reset remains available only if product decision allows emergency reset.

Plan decision:

- Keep Reset available because the functionality document lists Reset as visible while running/paused.
- Hide Pause and Skip only during work sessions when strict mode is on.

### 13.5 Auto-Start

Auto-start Work:

- When a break ends, automatically start a work session.

Auto-start Breaks:

- When a work session ends, automatically start a short or long break.

### 13.6 Notifications

Desktop Notifications:

- Send completion notifications.

Sound Notifications:

- Play completion chime.

Both settings must apply immediately.

## 14. Appearance Settings Page Plan

### 14.1 Source

Primary source:

- `Targets/Target_ui/appearance_tab.png`

### 14.2 Sections

Section 1:

- Title: `Bar Dimensions`.
- Subtitle: `Size of the progress bars on the top panel`.
- Icon: layout/dimensions.

Rows:

- Bar Width.
- Bar Height.
- Bar Corner Radius.

Section 2:

- Title: `Colors`.
- Subtitle: `Customize the glassmorphism bar colors`.
- Icon: palette.

Rows:

- Work Bar Color.
- Break Bar Color.
- Idle Pulse Color.

Cards:

- Color Saturation slider.
- Glow Intensity slider.

### 14.3 Bar Dimensions

Bar Width:

- Default: 140.
- Range: 60 to 300.
- Unit: pixels.

Bar Height:

- Screenshot displays default 4 and range 4 to 12.
- Functionality document displays default 3 and range 2 to 8.
- Build target should use screenshot values if this is a native rebuild.
- If GNOME extension compatibility is required, update the schema to match the visible UI.

Bar Corner Radius:

- Default: 99.
- Range: 0 to 99.
- 0 means rectangle.
- 99 means capsule.

### 14.4 Color Controls

Work Bar Color:

- Opens color picker.
- Updates focus progress arc and GNOME work bar.

Break Bar Color:

- Opens color picker.
- Updates break progress arc and GNOME break bar.

Idle Pulse Color:

- Opens color picker.
- Updates idle pulse visuals.

Use large circular color swatches on the right, as shown in the screenshot.

### 14.5 Slider Cards

Color Saturation:

- Range: 0.0 to 2.0.
- Step: 0.1.
- Default: 1.0.
- Visual: violet slider.

Glow Intensity:

- Range: 0 to 20.
- Step: 1.
- Default: 3.
- Visual: cyan slider.

Both slider cards use:

- Icon circle.
- Title.
- Description.
- Current value.
- Minus button.
- Plus button.
- Slider track.

## 15. Analytics Page Plan

### 15.1 Source

Primary source:

- `Targets/Target_ui/analytics_tab.png`

Functional source:

- Analytics section in `polindora_everything.md`

### 15.2 Screenshot Additions

The screenshot adds a richer analytics dashboard than the functionality document:

- Overview cards with trends.
- Timeframe dropdown.
- Daily / Weekly / Monthly segmented chart mode.
- Focus Time bar chart with labeled bars.
- Focus Distribution donut chart.
- Best Focus Time line chart.
- Session History list.

These are required.

### 15.3 Page Intro

Title:

- `Analytics`

Subtitle:

- `Track your productivity and focus journey`

Icon:

- Bar chart in violet circle.

### 15.4 Overview Section

Header:

- `Overview`

Timeframe dropdown:

- `Today`
- `This Week`
- `This Month`
- `All Time`

Default:

- `This Week`, matching screenshot.

Cards:

- Sessions.
- Focus Time.
- Completion Rate.
- Day Streak.

Each card includes:

- Icon.
- Main value.
- Label.
- Trend delta vs previous period.

Examples:

- `12 Sessions`
- `9h 32m Focus Time`
- `82% Completion Rate`
- `3 Day Streak`

Trend calculation:

- Compare selected period to equivalent previous period.
- Show positive deltas in green.
- Show negative deltas in red or muted warning.
- Show no-data state when previous period has no data.

### 15.5 Focus Time Chart

Visual:

- Large glass panel.
- Header `Focus Time`.
- Segmented control: Daily, Weekly, Monthly.
- Bar chart with glowing purple/blue bars.
- Value labels above bars.
- Axis labels.

Daily mode:

- Last 7 days.
- Bars by weekday.

Weekly mode:

- Last 4 to 8 weeks.
- Bars by week range.

Monthly mode:

- Last 6 to 12 months.
- Bars by month.

Interaction:

- Hover shows tooltip.
- Click selects bar.
- Selected bar filters Focus Distribution to the same period.

### 15.6 Focus Distribution

Visual:

- Donut chart.
- Center total time.
- Legend with category names, time, and percentages.

Data:

- Category minutes from completed work sessions.
- Include break time only if the selected chart mode explicitly asks for total schedule time.

Screenshot shows:

- Deep Work.
- Light Work.
- Break Time.

Plan:

- Support break time as a separate distribution category for analytics display.
- Keep focus-only charts separate where accuracy matters.

### 15.7 Best Focus Time

Visual:

- Card with title `Best Focus Time`.
- Main result like `10:00 AM - 12:00 PM`.
- Subtitle explaining productivity during that time.
- Small line chart by time of day.

Computation:

- Bucket completed work sessions by local hour.
- Sum focus minutes in each bucket.
- Smooth adjacent buckets to identify best 2-hour window.
- Require at least a minimum sample size before declaring a best time.

Fallback:

- If not enough data, show `Not enough data yet`.

### 15.8 Session History

Visual:

- Glass panel.
- Rows with icon, session title, timestamp, duration, status badge, chevron.

Row fields:

- Session type.
- Category.
- Started time.
- Duration.
- Status.

Examples:

- `Deep Work Session`
- `Short Break`
- `Completed`

Interactions:

- Click row opens session detail.
- Detail can show linked tasks, category, exact start/end time, and notes in future.

### 15.9 Reset Actions

The functionality document includes reset buttons. The screenshot does not show them in the first viewport, but they should exist lower in the page.

Actions:

- Reset Today's Stats.
- Reset All History.

Requirements:

- Use confirmation dialogs.
- Clearly describe what will be deleted.
- Never reset settings or tasks unless explicitly included.

## 16. Tasks Page Plan

### 16.1 Source

Primary source:

- `Targets/Target_ui/tasks_tab.png`

Functional source:

- Task Management section in `polindora_everything.md`

### 16.2 Screenshot Additions

The screenshot adds:

- Task metric cards.
- Priority labels.
- Due dates.
- Category filter.
- Sort control.
- Active task row action menu.
- Completed task preview section.
- `View all active tasks`.
- `View all completed`.

These are required.

### 16.3 Page Intro

Title:

- `Tasks`

Subtitle:

- `Organize your tasks and stay focused.`

Icon:

- Checklist in violet circle.

### 16.4 Task Metric Cards

Cards:

- Total Tasks.
- Completed.
- Remaining.
- Completion Rate.

Each card includes:

- Icon.
- Main number.
- Label.
- Description.

Completion rate formula:

```text
completed / (completed + active + deleted_incomplete)
```

This preserves the functionality document's behavior where deleting incomplete tasks impacts completion rate.

### 16.5 Create New Task Panel

Fields:

- Task name.
- Category.
- Priority.
- Due date.
- Add Task button.

Screenshot shows task name and category plus Add Task. Priority and due date are needed because active rows display them.

Implementation plan:

- Keep first viewport close to screenshot by showing task name and category prominently.
- Add priority and due date as compact optional controls, possibly revealed under an expand affordance if space is tight.

Category:

- Use dropdown with existing categories and option to create a new category.
- Empty category becomes `General`.

Priority:

- Low.
- Medium.
- High.
- Default: Medium.

Due date:

- Optional.

Add Task:

- Disabled until task name is non-empty.
- Creates task and clears form.

### 16.6 Active Tasks Panel

Header:

- `Active Tasks`.

Controls:

- Category filter.
- Sort control.

Category filter:

- All Categories.
- One option per category.

Sort control:

- Newest.
- Oldest.
- Priority.
- Due date.

Rows:

- Status circle.
- Task title.
- Category pill.
- Due date.
- Priority label.
- Row menu.

Priority colors:

- High: violet/pink.
- Medium: green/cyan.
- Low: blue.

Row actions:

- Mark complete.
- Edit.
- Delete.
- Change priority.
- Change due date.

Clicking status circle:

- Marks task complete.

### 16.7 Completed Tasks Panel

Header:

- `Completed Tasks`.

Header action:

- `View all completed`.

Rows:

- Completed check icon.
- Struck-through title.
- Completion or due date.
- Priority label.

Actions:

- Undo completion.
- Delete.

### 16.8 Top Tasks Integration

Home page Top Tasks should read from the same task service.

Top task selection:

1. Active tasks in selected category.
2. Sort by priority then due date then created date.
3. Show up to two tasks on Home page.

### 16.9 Pomodoro Task Tracking

When a work session completes:

- Read active category.
- Find active tasks with matching category.
- Increment `pomodoros_spent`.
- Update task row metadata.
- Include linked task IDs in session history.

## 17. Theme Page Plan

### 17.1 Source

Primary source:

- `Targets/Target_ui/Theme_tab.png`

Functional source:

- Themes section in `polindora_everything.md`

### 17.2 Screenshot Additions

The screenshot adds:

- Current theme panel.
- Theme preview cards.
- Active badge.
- Auto Switch.
- Accent color swatches.
- Light Mode.

These are required.

### 17.3 Page Intro

Title:

- `Theme`

Subtitle:

- `Customize the look and feel of Polindora.`

Icon:

- Palette in violet circle.

### 17.4 Current Theme Panel

Content:

- Theme preview orb.
- Theme name.
- Theme description.
- Active badge.

Example:

- `Polindora Dark`
- `The default dark theme for a focused experience.`

### 17.5 Theme Cards

Theme cards:

- Polindora Dark.
- Midnight Blue.
- Ocean Breeze.
- Sunset Glow.
- Aurora.
- Light Mode.

Each card includes:

- Preview image.
- Theme name.
- Description.
- Radio/check indicator.
- Active selected border.

Interaction:

- Click card applies theme immediately.
- Theme change animates palette values.
- Persist selected theme.

### 17.6 Auto Switch

Auto Switch row:

- Automatically switch theme based on system appearance.

Behavior:

- If enabled, read system dark/light preference.
- Use Polindora Dark or selected dark theme for dark mode.
- Use Light Mode for light mode unless user configures another mapping.

Setting:

- `theme_auto_switch: bool`

### 17.7 Accent Colors

Accent row:

- Pick favorite accent color.

Swatches:

- Violet.
- Blue.
- Cyan.
- Green.
- Amber.
- Orange.
- Pink.
- Custom/rainbow.

Behavior:

- Accent updates buttons, progress arcs, active tab, slider highlights, and chart colors.
- Custom/rainbow opens color picker.
- Persist as `accent_color`.

### 17.8 Relationship To Appearance Colors

Appearance page controls top-panel progress colors. Theme page controls app-wide palette and accent.

Implementation rule:

- Theme selection may update default work/break/idle colors.
- Manual Appearance color changes should mark progress colors as customized.
- Changing theme should ask whether to keep custom progress colors if they differ from the theme preset.

## 18. GNOME Panel Integration Plan

### 18.1 Source

Functional source:

- Top Bar UI sections in `polindora_everything.md`

### 18.2 Required Panel Features

Panel work bar:

- Right of clock.
- Shows work session progress.
- Uses work color.
- Shows remaining time label.

Panel break bar:

- Left of clock.
- Shows break progress.
- Uses break color.
- Shows remaining time label.

Panel icon:

- Status area icon.
- Click opens native app.
- Active state changes by timer state.

### 18.3 State Sharing

Preferred:

- Native app publishes state over D-Bus.
- Extension subscribes or polls lightweight D-Bus API.

Fallback:

- Native app writes state to GSettings.
- Extension watches GSettings changes.

Published state:

- `timer_state`
- `remaining_secs`
- `duration_secs`
- `progress`
- `is_paused`
- `work_bar_color`
- `break_bar_color`
- `idle_bar_color`
- `bar_width`
- `bar_height`
- `bar_radius`
- `glow_intensity`
- `color_saturation`

### 18.4 Panel Interaction

Single click on bar:

- Pause/resume.

Double click on bar:

- Open app.

Click panel icon:

- Open app.

The extension must send commands back to the native app through D-Bus or command GSettings.

### 18.5 Avoided Duplication

The extension must not own:

- Timer completion rules.
- Analytics updates.
- Task updates.
- Auto-start logic.

It only renders and sends user commands.

## 19. Notifications And Sounds

### 19.1 Desktop Notifications

Events:

- Work complete.
- Break complete.
- Long break complete.

Notification examples:

- `Pomodoro Complete!`
- `Break Over!`

Settings:

- Controlled by Desktop Notifications toggle.

### 19.2 Sound Notifications

Events:

- Work complete.
- Break complete.

Settings:

- Controlled by Sound Notifications toggle.

Implementation options:

- Qt multimedia sound.
- Freedesktop sound theme.
- GNOME sound player for extension integration.

Use the native Linux notification system where possible.

## 20. Animation Plan

### 20.1 Global Motion Rules

Animations should be smooth but functional.

Use:

- 120 to 180 ms for button hover/press.
- 180 to 260 ms for page transitions.
- 300 to 500 ms for chart transitions.
- 900 to 1800 ms for idle breathing effects.

Avoid:

- Slow transitions that block repeated use.
- Animation that changes layout dimensions unpredictably.

### 20.2 Timer Orb Animations

Required:

- Orb breathing glow in idle.
- Progress arc smoothing.
- Progress bead movement.
- Completion flash.
- Pause dim state.
- State transition pulse.

Completion:

- Gold/white flash.
- Slight orb scale pulse.
- Return to normal within 700 ms.

### 20.3 Button Animations

Required:

- Hover glow.
- Press scale.
- Disabled opacity.
- Active state border.

### 20.4 Settings Navigation

Required:

- Bottom tab active glow slides or fades.
- Page content crossfades or slides subtly.
- Search result focus highlights target row.

### 20.5 Charts

Required:

- Bars grow from baseline on first render.
- Donut chart sweeps in.
- Line chart draws from left to right.
- Hover tooltips fade in.

## 21. Accessibility And Input

### 21.1 Keyboard

Required:

- Tab navigation through controls.
- Enter/Space activates focused buttons.
- Arrow keys move segmented controls and bottom tabs.
- Escape closes search or settings.

### 21.2 Focus States

Every interactive element needs a visible focus state matching the glass/neon design.

### 21.3 Reduced Motion

If system reduced-motion setting is detected:

- Disable breathing animations.
- Shorten page transitions.
- Keep progress updates clear but less animated.

### 21.4 Contrast

Text must remain readable on glass panels.

Rules:

- Primary text should be high contrast.
- Secondary text should not fall below readable contrast.
- Colored labels need enough contrast against dark glass.

## 22. Build Phases

### Phase 1: Foundation

Goal:

- Create a running QML + Rust shell with persistent settings.

Tasks:

- Set up Cargo project.
- Add Qt/QML integration.
- Add app window.
- Add theme tokens.
- Add basic CXX-Qt bridge.
- Add SQLite initialization.
- Add settings repository.

Verification:

- App opens.
- Window renders.
- QML can read a Rust property.
- A setting persists after restart.

### Phase 2: Visual System

Goal:

- Build reusable glassmorphism components.

Tasks:

- Implement `GlassPanel`.
- Implement `GlassButton`.
- Implement `IconButton`.
- Implement `BottomTabBar`.
- Implement `StepperRow`.
- Implement `ToggleRow`.
- Implement `SliderCard`.
- Implement `StatCard`.
- Implement color tokens and theme manager.

Verification:

- Component gallery page or temporary dev view renders every component.
- Hover/press/focus states work.
- Text does not overflow at target width.

### Phase 3: Timer Engine

Goal:

- Timer behavior works independently of final UI.

Tasks:

- Implement timer state machine.
- Implement start/pause/resume/skip/reset.
- Implement short/long break decisions.
- Implement strict-mode capability flags.
- Add timer service tests.

Verification:

- Unit tests cover state transitions.
- Timer uses monotonic time.
- Pause/resume preserves elapsed time.

### Phase 4: Home Page

Goal:

- Build target home UI and connect it to timer state.

Tasks:

- Implement `TimerOrb`.
- Implement progress arc.
- Implement motivation quote area.
- Implement category dropdown.
- Implement Start/Pause/Resume/Skip/Reset controls.
- Implement Today and Top Tasks summary rows.
- Connect live timer state.

Verification:

- Home page visually matches screenshot.
- Timer counts down correctly.
- Strict mode affects controls.
- Summary rows navigate to settings pages.

### Phase 5: Settings Shell And Timer/Behavior/Appearance Pages

Goal:

- Build settings navigation and first three settings pages.

Tasks:

- Implement settings window/shell.
- Implement search overlay.
- Implement bottom tab navigation.
- Implement Timer page.
- Implement Behavior page.
- Implement Appearance page.
- Persist all controls.

Verification:

- All controls update settings.
- Active tab matches page.
- Search navigates to matching controls.
- Appearance changes update UI immediately.

### Phase 6: Tasks

Goal:

- Full task management.

Tasks:

- Implement task schema and repository.
- Implement task service.
- Implement task metrics.
- Implement task creation panel.
- Implement active tasks list.
- Implement completed tasks list.
- Implement filters and sort.
- Connect Home top tasks.

Verification:

- Create, complete, undo, delete work.
- Completion rate updates.
- Filters and sorting work.
- Completed work session increments matching task pomodoros.

### Phase 7: Analytics

Goal:

- Real analytics dashboard backed by sessions.

Tasks:

- Implement session persistence.
- Implement daily/weekly/monthly aggregation.
- Implement trend comparison.
- Implement streak calculation.
- Implement Focus Time chart.
- Implement Focus Distribution chart.
- Implement Best Focus Time chart.
- Implement Session History.
- Implement reset actions.

Verification:

- Analytics update after session completion.
- Charts handle empty state.
- Period filters work.
- Reset actions are confirmed and correct.

### Phase 8: Theme Page

Goal:

- Theme selection and accent customization.

Tasks:

- Implement theme service.
- Add screenshot theme presets.
- Implement current theme panel.
- Implement theme cards.
- Implement auto switch.
- Implement accent swatches.
- Connect theme to all components and charts.

Verification:

- Theme changes update app immediately.
- Accent changes update active controls and charts.
- Theme persists after restart.
- Light mode remains readable.

### Phase 9: Notifications, Sounds, And Background Behavior

Goal:

- Complete desktop productivity behavior.

Tasks:

- Add notifications.
- Add sounds.
- Add background timer behavior decision.
- Add close-to-tray or continue-in-background policy if needed.

Verification:

- Work complete notification fires when enabled.
- Break complete notification fires when enabled.
- Sound respects setting.
- Timer behavior after closing window is defined and tested.

### Phase 10: GNOME Panel Integration

Goal:

- Optional panel bars and icon.

Tasks:

- Create GNOME extension folder.
- Implement panel icon.
- Implement work and break bars.
- Implement D-Bus or GSettings bridge.
- Implement click commands.
- Implement appearance sync.

Verification:

- Bars appear around GNOME clock.
- Panel icon opens app.
- Progress matches native app.
- Pause/resume from panel works.

### Phase 11: Packaging

Goal:

- Build installable Linux package.

Tasks:

- Configure release build.
- Add `.desktop` file.
- Add app icon.
- Configure `cargo-deb`.
- Include QML, shaders, fonts, icons, and sounds.
- Include optional GNOME extension assets if enabled.

Verification:

- `cargo build --release` succeeds.
- `.deb` builds.
- Fresh install launches app.
- User data path is correct.

## 23. Testing Plan

### 23.1 Rust Unit Tests

Test:

- Timer transitions.
- Pause/resume elapsed calculation.
- Long break interval logic.
- Auto-start logic.
- Strict mode capability flags.
- Task completion rate.
- Analytics aggregation.
- Streak calculation.
- Best focus time calculation.
- Settings validation.

### 23.2 Integration Tests

Test:

- Completing a work session creates a session row.
- Completing a work session updates analytics.
- Completing a work session increments matching task pomodoros.
- Reset does not create completed analytics.
- Skip work does not count as completed.
- Skip break starts or returns to work according to rules.

### 23.3 UI Verification

Use visual screenshot checks for:

- Home page.
- Timer settings page.
- Behavior page.
- Appearance page.
- Analytics page.
- Tasks page.
- Theme page.

Viewports:

- Target compact desktop width.
- Slightly narrower minimum width.
- Taller desktop window.

Check:

- No overlapping text.
- Bottom tab labels fit.
- Cards do not resize unexpectedly on hover.
- Timer digits remain centered while counting.
- Charts are not blank.

### 23.4 Manual QA Checklist

Timer:

- Start work.
- Pause.
- Resume.
- Skip.
- Reset.
- Complete work.
- Complete break.
- Trigger long break.

Settings:

- Change every duration.
- Toggle every behavior option.
- Change every appearance control.
- Search every page.

Tasks:

- Add task with category.
- Add task with priority.
- Add task with due date.
- Complete task.
- Undo task.
- Delete active task.
- Filter and sort.

Analytics:

- Verify empty state.
- Verify populated state.
- Verify period switch.
- Verify chart selection.
- Verify reset actions.

Theme:

- Select every theme.
- Toggle auto switch.
- Select every accent.
- Pick custom accent.

Panel integration:

- Verify progress position.
- Verify click behavior.
- Verify appearance sync.

## 24. Implementation Risks

### 24.1 QML/Rust Bridge Complexity

Risk:

- CXX-Qt can be more complex than a pure QML or pure Rust UI.

Mitigation:

- Keep bridge objects coarse-grained.
- Expose view models instead of many tiny properties.
- Keep domain logic pure Rust and testable.

### 24.2 GNOME Panel Integration Scope

Risk:

- GNOME Shell extension work is separate from native app work.

Mitigation:

- Build the native app first.
- Treat panel integration as a later phase.
- Do not duplicate timer logic in the extension.

### 24.3 Glassmorphism Performance

Risk:

- Too many blur/shader layers can hurt performance.

Mitigation:

- Use reusable lightweight effects.
- Cache static layers.
- Avoid excessive live blur.
- Profile on target hardware.

### 24.4 Analytics Data Growth

Risk:

- Session history can grow indefinitely.

Mitigation:

- Keep sessions indexed.
- Aggregate for charts.
- Avoid loading full history into QML.

### 24.5 UI Screenshot Fidelity

Risk:

- The screenshots are stylized and may not map exactly to native controls.

Mitigation:

- Build custom QML controls.
- Use screenshot layout as the visual target.
- Prefer functional clarity where a screenshot detail conflicts with app behavior.

## 25. Final Build Order Recommendation

Recommended order:

1. Foundation app shell.
2. Visual component system.
3. Timer engine.
4. Home page.
5. Settings shell.
6. Timer, Behavior, Appearance pages.
7. Tasks.
8. Analytics.
9. Theme system.
10. Notifications and background behavior.
11. GNOME panel integration.
12. Packaging.

This order keeps the core app usable early and delays the GNOME extension integration until the timer state and appearance settings are stable.
