# POLINDORA — Technical Stack & Architecture Document

## Project Vision

Polindora is a **premium Linux productivity application** focused on:

- Pomodoro workflow
- Strong fluid UI
- Modern glassmorphism
- Smooth animations
- Premium desktop experience
- Native Linux performance
- Ubuntu (`.deb`) deployment

### Visual Direction

> **Dark saturated glassmorphism + fluid motion + premium desktop UX**

Inspired by:

- VisionOS
- macOS liquid glass
- Modern gaming dashboards
- Futuristic productivity tools

---

# Why We Chose This Stack

The app has extremely specific requirements.

We need:

- ✓ Strong glassmorphism
- ✓ GPU accelerated visuals
- ✓ Blur & glow effects
- ✓ Custom animated components
- ✓ Native Linux app
- ✓ Excellent performance
- ✓ Modern architecture
- ✓ Easy `.deb` packaging
- ✓ Long-term scalability

This rules out many conventional approaches.

---

# Final Technology Stack

| Layer              | Technology                                                             |
| ------------------ | ---------------------------------------------------------------------- |
| **Frontend**       | QML + QtQuick                                                          |
| **Backend Logic**  | Rust                                                                   |
| **Bridge**         | CXX-Qt                                                                 |
| **Animations**     | QtQuick Animation System                                               |
| **Visual Effects** | ShaderEffect, MultiEffect, DropShadow, OpacityMask, Canvas, Gradients, Layer Effects |
| **Async Runtime**  | Tokio                                                                  |
| **Persistence**    | SQLite + SQLx                                                          |
| **Assets**         | SVG, Fonts, GLSL Shaders                                               |
| **Packaging**      | cargo-deb, dpkg, AppImage (optional)                                   |
| **Platform**       | Linux / Ubuntu                                                         |

---

# 1. UI Framework — QML + QtQuick

## Purpose

QML powers:

- UI layouts
- Component system
- Animations
- Interactions
- Visual hierarchy
- Glassmorphism layer
- Reactive interface logic

## Why QML?

Polindora is not a standard form application. It needs:

- Glowing timer sphere
- Animated progress rings
- Premium controls
- Fluid cards
- Glossy highlights
- Motion-rich interactions

Traditional Linux toolkits struggle here. QML excels at:

- Custom visuals
- GPU rendering
- Animations
- Shader-driven UI
- Modern interfaces

## Why NOT GTK?

GTK is excellent for native GNOME apps and traditional desktop UI. But it is weaker for:

- Heavy glassmorphism
- Fluid motion systems
- Complex visual effects

Our design language depends heavily on layered opacity, animated glow, and custom rendering. QML fits this much better.

## Why NOT Electron / Tauri?

Web stacks provide excellent visuals. However we want:

- Native Linux feel
- Lower memory usage
- Tighter graphics integration
- Direct GPU effects
- No browser runtime

Qt provides these advantages over web-based approaches.

## QML Responsibilities

QML controls:

- Main window
- Navigation
- Tabs
- Settings pages
- Cards
- Buttons
- Progress bars
- Glass containers
- Theme system
- Transitions
- Micro-interactions

### Example Components

```
TimerSphere.qml
GlassButton.qml
ProgressRing.qml
HeaderBar.qml
TaskCard.qml
ToggleSwitch.qml
SliderControl.qml
ThemeManager.qml
```

---

# 2. Backend Language — Rust

## Purpose

Rust powers:

- Business logic
- Timer engine
- Task management
- Analytics
- State management
- Persistence layer
- Async jobs

## Why Rust?

We evaluated Python, C++, and Rust. Rust was selected for the following reasons.

### Native Performance

Compiled binaries with very low runtime overhead. Good for timers, animation coordination, async work, and background processing.

### Memory Efficiency

Compared to Python, Rust has a significantly smaller runtime footprint. This is important for desktop responsiveness.

### Strong Safety Model

Rust provides:

- Ownership
- Borrow checking
- Thread safety
- Memory safety

This results in fewer runtime bugs and excellent long-term maintainability.

### Concurrency

Rust enables real multithreading using Tokio, channels, and async tasks. No Python GIL limitations.

### Packaging Advantages

Rust integrates beautifully with Linux packaging. Simple release flow:

```bash
cargo build --release
cargo deb
```

Clean native deployment.

## Why NOT Python?

Python would have worked. However we want:

- Native Linux desktop architecture
- High scalability
- Minimal memory overhead
- Long-term maintainability

Rust fits these goals better.

---

# 3. Rust ↔ QML Communication — CXX-Qt

## Purpose

Bridge between the QML frontend and the Rust backend.

### Architecture

```
QML UI
    ↓
QObject Properties
Signals / Slots
    ↓
CXX-Qt
    ↓
Rust Logic
```

## Why CXX-Qt?

We need strong Qt integration. CXX-Qt provides:

- ✓ QObject support
- ✓ Q_PROPERTY support
- ✓ Signals / slots
- ✓ QML compatibility
- ✓ Modern Rust ergonomics

### Example Usage

Rust exposes properties such as:

- `remainingSeconds`
- `isRunning`
- `themeMode`
- `activeTasks`

QML automatically reacts to changes. This creates a **reactive native UI architecture**.

---

# 4. Animation System — QtQuick Animations

Polindora heavily relies on motion. Animation is a first-class system.

## Tools Used

- `Behavior`
- `NumberAnimation`
- `ParallelAnimation`
- `SequentialAnimation`
- `SpringAnimation`
- `OpacityAnimator`
- `RotationAnimator`

## Used For

### Timer Orb

- Breathing glow
- Pulse
- Hover response

### Buttons

- Hover transitions
- Press response
- Lighting changes

### Progress Ring

- Animated arc updates

### Navigation

- Active tab transitions

### Settings Controls

- Slider fluidity
- Toggle transitions

## Why QtQuick Animation?

GPU accelerated. Extremely smooth. Purpose-built for modern interfaces.

---

# 5. Visual Effects System

This is the heart of Polindora's design language.

## ShaderEffect

Used for:

- Glass highlights
- Liquid reflections
- Orb sheen
- Soft shimmer
- Custom glow

### Why ShaderEffect?

Our UI aesthetic requires strong premium rendering. Standard rectangles are insufficient. ShaderEffect allows custom fragment shaders — perfect for VisionOS-style visuals.

## MultiEffect

Used for:

- Blur
- Brightness
- Saturation
- Soft glow

## DropShadow

Used carefully for:

- Depth
- Floating layers
- Button elevation
- Glass edge lighting

## OpacityMask

Used for:

- Masked gradients
- Orb clipping
- Shape blending

## Canvas

Used for:

- Custom progress ring rendering
- Dynamic arcs
- Advanced drawing

## Gradient System

Used everywhere. Provides:

- Glass surfaces
- Lighting simulation
- Deep backgrounds
- Accent glow

## Layer Effects

Used for:

- Visual composition
- Stacked glass surfaces
- Effect isolation

---

# 6. Async Runtime — Tokio

Used for:

- Background jobs
- Notifications
- Autosave
- Timers
- Future sync services

### Why Tokio?

Fast. Modern. Battle-tested. Excellent Rust async ecosystem.

---

# 7. Database Layer — SQLite + SQLx

Used for:

- Settings
- Task storage
- Pomodoro history
- Analytics
- Themes
- Statistics

### Why SQLite?

Perfect desktop database. No external server. Single file storage. Reliable.

### Why SQLx?

Modern Rust DB toolkit. Compile-time query validation. Async friendly.

---

# 8. Assets System

### SVG Icons

Chosen because they are:

- Resolution independent
- Lightweight
- Themeable
- Clean rendering

### Fonts

Custom typography for premium UI identity.

### GLSL Shaders

Used for advanced glass visuals.

---

# 9. Packaging & Distribution — cargo-deb

Target platform: **Ubuntu / Linux**.

### Build Flow

```bash
cargo build --release
cargo deb
```

### Output

Primary: `.deb` installer

Optional future outputs:

- AppImage
- Flatpak
- Snap

### Why cargo-deb?

Native Rust packaging. Simple deployment pipeline. Linux friendly.

---

# 10. Folder Structure

Recommended project structure:

```
polindora/

├── Cargo.toml

├── src/
│   ├── main.rs
│   ├── bridge.rs
│   ├── timer_engine.rs
│   ├── task_store.rs
│   ├── analytics.rs
│   └── state.rs

├── qml/
│   ├── Main.qml
│   ├── Theme.qml
│   ├── components/
│   │   ├── TimerSphere.qml
│   │   ├── GlassButton.qml
│   │   ├── Toggle.qml
│   │   ├── SettingsRow.qml
│   │   └── ProgressRing.qml

├── shaders/

├── assets/
│   ├── icons/
│   ├── fonts/
│   └── gradients/

└── resources.qrc
```

---

# Final Philosophy

Polindora's stack was selected around one core principle:

> **Deliver a premium native Linux productivity application with strong fluid glassmorphism and high performance.**

Our stack intentionally combines:

| Technology               | Role                             |
| ------------------------ | -------------------------------- |
| **QML**                  | Expressive GPU-driven UI         |
| **Rust**                 | Safe, scalable, native logic     |
| **Qt Effects + Shaders** | Premium visual rendering         |
| **SQLite + Tokio**       | Robust desktop functionality     |

This architecture is designed to support both **beautiful UX** and **serious production-grade engineering**.
