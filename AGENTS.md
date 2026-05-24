# AGENTS.md

This file is the operating contract for AI agents working in the Polindora root
directory. It is intended to be compatible with OpenAI Codex containerized task
execution and Google Antigravity 2.0 style Agent-to-Agent (A2A) parallel
execution.

Polindora is a premium Linux productivity app for Pomodoro workflows. The user
experience should feel fluid, dark, glassy, responsive, and polished without
compromising native performance or maintainability.

## Baseline Engineering Guide

These rules come from the original project agent guide and remain in force for
all agents and subagents.

### 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:

- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them; don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

### 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No flexibility or configurability that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes,
simplify.

### 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:

- Don't improve adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it; don't delete it.

When your changes create orphans:

- Remove imports, variables, and functions that your changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

### 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:

- "Add validation" means write tests for invalid inputs, then make them pass.
- "Fix the bug" means write a test that reproduces it, then make it pass.
- "Refactor X" means ensure tests pass before and after.

For multi-step tasks, state a brief plan:

```text
1. [Step] -> verify: [check]
2. [Step] -> verify: [check]
3. [Step] -> verify: [check]
```

Strong success criteria let agents loop independently. Weak criteria such as
"make it work" require clarification.

These guidelines are working if there are fewer unnecessary changes in diffs,
fewer rewrites due to overcomplication, and clarifying questions come before
implementation rather than after mistakes.

## 1. Global Working Rules

Before changing code, every agent must state:

- The task goal in one sentence.
- The assumptions being made.
- The exact files or directories expected to change.
- The verification command or manual check that proves the change works.

If a requirement is ambiguous, stop and ask before editing. Do not silently
choose between materially different interpretations.

Keep changes surgical:

- Touch only files required by the task.
- Match existing style and naming.
- Do not refactor adjacent code unless the task requires it.
- Do not add speculative hooks, options, abstractions, or future-proofing.
- Remove only unused code created by the current change.
- Never revert or overwrite user changes unless explicitly instructed.

Prefer the simplest correct implementation. If an implementation becomes larger
than the problem justifies, pause and simplify.

## 2. Project Architecture

Core stack:

- App type: Premium Linux Pomodoro productivity app.
- Frontend: QML, QtQuick, QtQuick Animation, ShaderEffect, MultiEffect, Canvas.
- Backend: Rust, Tokio async runtime, SQLite through SQLx.
- Bridge: CXX-Qt with QObject macros, properties, signals, and slots.
- Deployment: Ubuntu Linux package built with cargo-deb.

Directory ownership map:

- `src/`: Rust backend, application state, CXX-Qt bridge, async services.
- `qml/`: Main QML application views and global UI composition.
- `qml/components/`: Reusable QML controls and visual components.
- `shaders/`: Custom GLSL shader sources.
- `assets/`: Static images, icons, fonts, and package resources.
- `Cargo.toml`: Rust dependencies, features, metadata, and cargo-deb settings.
- `resources.qrc`: Qt resource registration for QML, shaders, and assets.

No agent may move files across these boundaries without Tech Lead approval.

## 3. Codex Container Execution Rules

When running inside OpenAI Codex:

- Inspect existing files before editing.
- Use `rg` or `rg --files` for discovery.
- Use `apply_patch` for manual edits.
- Avoid destructive commands such as `rm`, `git reset`, and `git checkout --`
  unless the user explicitly requests them.
- Do not install dependencies or access the network unless the task requires it
  and approval has been granted.
- Run the narrowest useful verification first, then broader checks when the
  touched surface justifies it.
- Report commands that could not be run and why.

Expected verification ladder:

1. Rust-only logic change: `cargo test` or the narrowest relevant test target.
2. Bridge or build-system change: `cargo build`.
3. Release or packaging change: `cargo build --release`, then `cargo deb`.
4. QML visual change: run the app when possible and inspect the affected view.
5. Shader change: verify rendering on native Linux when possible and check for
   frame drops or visual artifacts.

## 4. Antigravity A2A Parallel Execution Rules

When running in an A2A-capable multi-agent environment, Antigravity should split
work by ownership boundary and run independent agents asynchronously.

Every A2A task packet must include:

- `goal`: User-visible result expected from the task.
- `owner`: One specialized subagent.
- `scope`: Exact file paths the agent may edit.
- `inputs`: Bridge properties, QML contracts, schema assumptions, or assets
  required from other agents.
- `outputs`: Files changed, public APIs changed, signals/properties added, and
  verification performed.
- `handover`: Clear next action for dependent agents.

Parallelism rules:

- Agents may work in parallel only when their edit scopes do not overlap.
- If two agents need the same file, the Tech Lead owns sequencing.
- `src/bridge.rs`, `Cargo.toml`, and `resources.qrc` are coordination choke
  points and must not be edited concurrently.
- Public CXX-Qt properties, signals, slots, enum names, and QML-visible method
  signatures are shared contracts. Any change to them requires a handover note.
- The QA & Packaging DevOps Specialist is automatically invoked whenever a core
  Rust module, database schema, bridge property, `Cargo.toml`, or `resources.qrc`
  changes.

Backend feature workflow:

1. Rust Core & Core-Logic Engineer implements state, persistence, async runtime
   behavior, and tests in its owned files.
2. Tech Lead & System Architect updates `src/bridge.rs` and any required global
   resource or dependency declarations.
3. QML UI & Animation Engineer binds views/components to the new bridge
   properties and signals.
4. Graphics & GLSL Shader Engineer works in parallel only if the feature needs
   rendering effects or shader-backed surfaces.
5. QA & Packaging DevOps Specialist runs verification and reports failures back
   to the owning agent.

UI feature workflow:

1. QML UI & Animation Engineer defines the interaction and component behavior.
2. Tech Lead confirms bridge impact if new backend data is required.
3. Graphics & GLSL Shader Engineer supplies shader effects only when QML-native
   effects are insufficient.
4. QA validates layout, startup, and build behavior.

## 5. Specialized Subagents

### 5.1 Tech Lead & System Architect

Role:

- Owns system architecture, CXX-Qt bridge declarations, global app state, and
  project-wide configuration.
- Coordinates multi-agent sequencing and resolves ownership conflicts.

Primary scope:

- `src/bridge.rs`
- Shared Rust application state under `src/` when no narrower owner applies.
- `Cargo.toml`
- `resources.qrc`

Rules:

- Enforce explicit Rust-to-Qt error boundaries.
- Do not expose fallible Rust operations to QML without a clear success/error
  signal or state property.
- Keep CXX-Qt properties small, typed, and stable.
- Prefer immutable snapshots or explicit change signals over hidden mutable
  shared state.
- Ensure QObject properties, signals, and slots are named for QML clarity.
- Keep global dependencies minimal and justified.
- A clean `cargo build` is required after bridge or configuration changes.

Handover outputs:

- New or changed bridge properties, signals, slots, and methods.
- Required QML binding names and expected value semantics.
- Verification result for `cargo build` or reason it could not be run.

### 5.2 Rust Core & Core-Logic Engineer

Role:

- Implements native backend behavior, high-precision async timers, task storage,
  SQLite persistence, and analytics.

Strict scope:

- `src/timer_engine.rs`
- `src/task_store.rs`
- `src/analytics.rs`
- Database schema files if present, such as `migrations/` or SQLx metadata.

Rules:

- Use Tokio for non-blocking async execution.
- Do not block the async runtime with synchronous database or filesystem work.
- Prefer SQLx compile-time checked queries when project setup supports them.
- Keep background timers low CPU by sleeping until meaningful state changes.
- Persist state at explicit boundaries; avoid excessive write frequency.
- Model timer state transitions explicitly and test edge cases.
- Treat time drift, pause/resume, suspend/resume, and app restart as first-class
  correctness concerns.
- Do not edit QML, shaders, bridge declarations, packaging metadata, or assets.

Verification:

- Add or update focused Rust tests for changed timer, storage, or analytics
  behavior.
- Run `cargo test` or the narrowest relevant test target.
- If SQLx metadata or migrations change, verify database setup according to the
  repository's existing workflow.

Handover outputs:

- State fields or events that must be exposed through the bridge.
- Database migration notes and compatibility implications.
- Timer precision and CPU behavior assumptions.

### 5.3 QML UI & Animation Engineer

Role:

- Crafts the premium QML interface, fluid layouts, navigation transitions, and
  custom reactive component library.

Strict scope:

- `qml/Main.qml`
- `qml/Theme.qml`
- `qml/components/`

Rules:

- Do not use generic default controls when a custom Polindora control is needed.
- All interactive elements must include hover, press, focus, disabled, and
  active states where applicable.
- Use fluid micro-interactions, spring-style transitions, opacity changes,
  scale changes, and smooth easing without making the app feel slow.
- Follow tokens from `qml/Theme.qml` for color, spacing, radius, typography,
  glow, shadow, and animation durations.
- Keep components reusable only when reuse is real in the current task.
- Avoid hardcoded colors or timings outside `Theme.qml`.
- Do not edit Rust, shaders, packaging, or resource manifests unless the Tech
  Lead explicitly sequences that work.

Visual quality bar:

- Dark glassmorphism must remain legible and accessible.
- Text must not overflow or overlap at supported window sizes.
- Animations must not cause layout jitter.
- Components should be keyboard navigable where interaction requires it.

Verification:

- Run or inspect the affected QML view when possible.
- Confirm all new assets, shaders, or QML files are registered by the Tech Lead
  in `resources.qrc` before relying on them.

Handover outputs:

- QML properties consumed from the bridge.
- New component APIs and required Theme tokens.
- Any rendering effects requested from the shader owner.

### 5.4 Graphics & GLSL Shader Engineer

Role:

- Implements advanced visual effects, glass layers, liquid reflections, glow,
  blur-like surfaces, and shader-backed premium rendering.

Strict scope:

- `shaders/`
- QML rendering layers using `ShaderEffect` or `MultiEffect` when explicitly
  assigned by the Tech Lead.

Rules:

- Keep shaders GPU-friendly and compatible with native Linux Qt rendering.
- Avoid expensive loops, dynamic branching, and excessive texture sampling.
- Prefer QML-native `MultiEffect` when it provides the required effect at lower
  complexity.
- Uniform names must be stable and documented at the QML call site.
- Every shader must have a graceful fallback or a non-shader visual baseline
  when practical.
- Do not change business logic, storage, bridge declarations, or packaging.

Performance bar:

- No visible frame drops during standard timer interactions.
- Effects must not increase idle CPU usage meaningfully.
- Animations should be driven by existing frame clocks or QML animations, not
  ad hoc busy loops.

Verification:

- Inspect the affected view on native Linux where possible.
- Confirm shader resources are included through `resources.qrc`.
- Report any environment where GPU profiling could not be performed.

Handover outputs:

- Shader file names, uniforms, expected ranges, and fallback behavior.
- QML layer requirements for `ShaderEffect` or `MultiEffect`.

### 5.5 QA & Packaging DevOps Specialist

Role:

- Validates backend logic, bridge integration, QML layout behavior, and Debian
  packaging readiness.

Strict scope:

- Automated test suites.
- Integration tests.
- Debian packaging scripts and cargo-deb related build flow.
- Verification-only changes to CI or test configuration when present.

Rules:

- Do not change product behavior to make tests pass.
- Failures must be assigned to the owning agent with exact command output and
  suspected ownership.
- Verify `cargo build --release` before generating deployment artifacts.
- Run `cargo deb` only after release build success.
- Keep packaging metadata aligned with `Cargo.toml`, `resources.qrc`, assets,
  and Ubuntu desktop integration expectations.

Verification responsibilities:

- Rust unit and integration tests.
- Bridge build checks.
- QML resource inclusion checks.
- Release build checks.
- Debian package generation with `cargo deb`.

Handover outputs:

- Commands run.
- Pass/fail result.
- Failure owner and minimal reproduction command.
- Artifact path when package generation succeeds.

## 6. File Ownership Matrix

Only the listed owner may edit a path unless the Tech Lead explicitly sequences
an exception.

| Path | Primary owner | Notes |
| --- | --- | --- |
| `src/bridge.rs` | Tech Lead & System Architect | CXX-Qt contract choke point. |
| `src/timer_engine.rs` | Rust Core & Core-Logic Engineer | Async timer state and precision. |
| `src/task_store.rs` | Rust Core & Core-Logic Engineer | SQLite persistence and task data. |
| `src/analytics.rs` | Rust Core & Core-Logic Engineer | Tracking and derived metrics. |
| `src/` other files | Tech Lead & System Architect | Delegate only when scope is clear. |
| `qml/Main.qml` | QML UI & Animation Engineer | Main app composition. |
| `qml/Theme.qml` | QML UI & Animation Engineer | Global design tokens. |
| `qml/components/` | QML UI & Animation Engineer | Reusable custom controls. |
| `shaders/` | Graphics & GLSL Shader Engineer | GLSL sources and shader-specific docs. |
| `assets/` | QA & Packaging or Tech Lead | Assets must be registered and packaged. |
| `Cargo.toml` | Tech Lead & System Architect | Dependencies, features, package metadata. |
| `resources.qrc` | Tech Lead & System Architect | Qt resource registration. |
| Tests | QA & Packaging DevOps Specialist plus feature owner | Feature owner writes logic tests; QA validates. |

## 7. Bridge Contract Rules

CXX-Qt is the contract between Rust and QML. Treat it as a public API.

- Every QML-visible property must define ownership, update timing, and error
  semantics.
- Emit change signals only when values actually change.
- Do not expose raw database models directly to QML if a stable view model is
  more appropriate.
- Avoid long-running work in slots. Slots should enqueue async work or update
  cheap state.
- Convert backend failures into explicit QML-visible error state.
- Keep bridge naming consistent with QML usage.

Any bridge change triggers:

- Tech Lead review.
- QML handover if bindings are affected.
- QA verification with at least `cargo build`.

## 8. Rust Backend Rules

- Use Tokio tasks deliberately and keep cancellation paths explicit.
- Avoid shared mutable state unless protected and justified.
- Prefer typed errors internally; convert to user-facing messages at the bridge.
- Keep SQLite access behind storage APIs.
- Keep SQL migrations forward-only.
- Add tests for timer transitions, persistence edge cases, and analytics math.
- Measure or reason explicitly about idle CPU impact for background loops.

## 9. QML and Visual Design Rules

- The app should feel premium, dark, glassy, and responsive.
- Use `Theme.qml` tokens for visual consistency.
- Prefer custom QML components over generic controls.
- Use animations to clarify state changes, not to distract.
- Keep frame pacing smooth during timer updates.
- Avoid visual changes that reduce legibility.
- Do not introduce text, buttons, or panels that overflow at common Linux
  desktop window sizes.

## 10. Shader Rules

- Shaders must serve a visible product purpose.
- Keep uniform contracts small and stable.
- Provide reasonable defaults for all uniforms.
- Use precision and sampling choices appropriate for desktop Linux GPUs.
- Do not implement effects in shaders when standard QtQuick effects are enough.
- Verify resource registration before considering shader work complete.

## 11. Packaging Rules

- `cargo build --release` must pass before `cargo deb`.
- Debian package metadata must remain consistent with `Cargo.toml`.
- Assets required at runtime must be present in `assets/` and registered through
  `resources.qrc` when used by Qt.
- Packaging changes must not hide build warnings or test failures.
- Generated artifacts should not be committed unless the repository already
  tracks them.

## 12. Completion Standard

A task is complete only when:

- The requested behavior or file change is implemented.
- Ownership boundaries were respected.
- The narrowest relevant verification was run, or the reason it could not run
  is reported.
- Affected bridge, QML, shader, asset, and packaging contracts are documented in
  the handover or final response.
- No unrelated files were changed.
