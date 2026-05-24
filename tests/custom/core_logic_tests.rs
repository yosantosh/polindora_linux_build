#![allow(dead_code)]

#[path = "../../src/analytics.rs"]
mod analytics;

#[path = "../../src/timer_engine.rs"]
mod timer_engine;

use analytics::{summarize_sessions, SessionKind, SessionRecord};
use timer_engine::{TimerConfig, TimerEngine, TimerPhase};

#[test]
fn timer_uses_long_break_after_configured_interval() {
    let mut timer = TimerEngine::new(TimerConfig {
        long_break_interval: 3,
        ..TimerConfig::default()
    });

    let mut next = TimerPhase::Idle;
    for _ in 0..3 {
        timer.start_work();
        next = timer.skip();
    }

    assert_eq!(next, TimerPhase::LongBreak);
    assert_eq!(timer.snapshot().completed_in_cycle, 3);
}

#[test]
fn idle_timer_config_update_changes_default_remaining_time() {
    let mut timer = TimerEngine::new(TimerConfig::default());

    timer.set_config(TimerConfig {
        work_seconds: 45 * 60,
        ..TimerConfig::default()
    });

    let snapshot = timer.snapshot();
    assert_eq!(snapshot.phase, TimerPhase::Idle);
    assert_eq!(snapshot.remaining_seconds, 45 * 60);
}

#[test]
fn strict_mode_is_exposed_in_timer_snapshot() {
    let timer = TimerEngine::new(TimerConfig {
        strict_mode: false,
        ..TimerConfig::default()
    });

    assert!(!timer.snapshot().strict_mode);
}

#[test]
fn analytics_ignores_break_sessions_for_focus_totals() {
    let summary = summarize_sessions(&[
        SessionRecord {
            started_day: "2026-05-24".to_string(),
            started_hour: 9,
            kind: SessionKind::Work,
            completed: true,
            duration_seconds: 25 * 60,
            category: "General".to_string(),
        },
        SessionRecord {
            started_day: "2026-05-24".to_string(),
            started_hour: 10,
            kind: SessionKind::LongBreak,
            completed: true,
            duration_seconds: 15 * 60,
            category: "Break".to_string(),
        },
    ]);

    assert_eq!(summary.completed_work_sessions, 1);
    assert_eq!(summary.focus_minutes, 25);
    assert_eq!(summary.category_minutes.get("General"), Some(&25));
    assert_eq!(summary.category_minutes.get("Break"), None);
}

#[test]
fn analytics_selects_best_focus_hour_by_completed_work_seconds() {
    let summary = summarize_sessions(&[
        SessionRecord {
            started_day: "2026-05-24".to_string(),
            started_hour: 9,
            kind: SessionKind::Work,
            completed: true,
            duration_seconds: 25 * 60,
            category: "Planning".to_string(),
        },
        SessionRecord {
            started_day: "2026-05-24".to_string(),
            started_hour: 14,
            kind: SessionKind::Work,
            completed: true,
            duration_seconds: 50 * 60,
            category: "Build".to_string(),
        },
    ]);

    assert_eq!(summary.best_focus_hour, Some(14));
}
