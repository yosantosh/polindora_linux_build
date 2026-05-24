use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerPhase {
    Idle,
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerConfig {
    pub work_seconds: u32,
    pub short_break_seconds: u32,
    pub long_break_seconds: u32,
    pub long_break_interval: u32,
    pub strict_mode: bool,
    pub auto_start_breaks: bool,
    pub auto_start_work: bool,
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            work_seconds: 25 * 60,
            short_break_seconds: 5 * 60,
            long_break_seconds: 15 * 60,
            long_break_interval: 4,
            strict_mode: true,
            auto_start_breaks: false,
            auto_start_work: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimerSnapshot {
    pub phase: TimerPhase,
    pub remaining_seconds: u32,
    pub total_seconds: u32,
    pub progress: f32,
    pub is_paused: bool,
    pub strict_mode: bool,
    pub completed_in_cycle: u32,
}

#[derive(Debug)]
pub struct TimerEngine {
    config: TimerConfig,
    phase: TimerPhase,
    started_at: Option<Instant>,
    paused_at: Option<Instant>,
    accumulated_pause: Duration,
    total_duration: Duration,
    completed_in_cycle: u32,
}

impl TimerEngine {
    pub fn new(config: TimerConfig) -> Self {
        Self {
            config,
            phase: TimerPhase::Idle,
            started_at: None,
            paused_at: None,
            accumulated_pause: Duration::ZERO,
            total_duration: Duration::from_secs(config.work_seconds.into()),
            completed_in_cycle: 0,
        }
    }

    pub fn start_work(&mut self) {
        self.start_phase(TimerPhase::Work, self.config.work_seconds);
    }

    pub fn config(&self) -> TimerConfig {
        self.config
    }

    pub fn set_config(&mut self, config: TimerConfig) {
        self.config = config;

        if self.phase == TimerPhase::Idle {
            self.total_duration = Duration::from_secs(config.work_seconds.into());
        }
    }

    pub fn start_short_break(&mut self) {
        self.start_phase(TimerPhase::ShortBreak, self.config.short_break_seconds);
    }

    pub fn start_long_break(&mut self) {
        self.start_phase(TimerPhase::LongBreak, self.config.long_break_seconds);
    }

    pub fn pause(&mut self) -> bool {
        if self.phase == TimerPhase::Idle || self.paused_at.is_some() {
            return false;
        }

        self.paused_at = Some(Instant::now());
        true
    }

    pub fn resume(&mut self) -> bool {
        let Some(paused_at) = self.paused_at.take() else {
            return false;
        };

        self.accumulated_pause += paused_at.elapsed();
        true
    }

    pub fn reset(&mut self) {
        self.phase = TimerPhase::Idle;
        self.started_at = None;
        self.paused_at = None;
        self.accumulated_pause = Duration::ZERO;
        self.total_duration = Duration::from_secs(self.config.work_seconds.into());
    }

    pub fn skip(&mut self) -> TimerPhase {
        self.complete_current_phase()
    }

    pub fn tick(&mut self) -> Option<TimerPhase> {
        if self.phase == TimerPhase::Idle || self.paused_at.is_some() {
            return None;
        }

        if self.remaining_duration().is_zero() {
            return Some(self.complete_current_phase());
        }

        None
    }

    pub fn snapshot(&self) -> TimerSnapshot {
        let remaining = self.remaining_duration();
        let total_seconds = self.total_duration.as_secs().min(u32::MAX as u64) as u32;
        let remaining_seconds = display_seconds(remaining);
        let elapsed_seconds = total_seconds.saturating_sub(remaining_seconds);
        let progress = if total_seconds == 0 {
            0.0
        } else {
            elapsed_seconds as f32 / total_seconds as f32
        };

        TimerSnapshot {
            phase: self.phase,
            remaining_seconds,
            total_seconds,
            progress,
            is_paused: self.paused_at.is_some(),
            strict_mode: self.config.strict_mode,
            completed_in_cycle: self.completed_in_cycle,
        }
    }

    fn start_phase(&mut self, phase: TimerPhase, duration_seconds: u32) {
        self.phase = phase;
        self.started_at = Some(Instant::now());
        self.paused_at = None;
        self.accumulated_pause = Duration::ZERO;
        self.total_duration = Duration::from_secs(duration_seconds.into());
    }

    fn complete_current_phase(&mut self) -> TimerPhase {
        let completed_phase = self.phase;

        match completed_phase {
            TimerPhase::Work => {
                self.completed_in_cycle += 1;
                let next_phase = if self.completed_in_cycle % self.config.long_break_interval == 0 {
                    TimerPhase::LongBreak
                } else {
                    TimerPhase::ShortBreak
                };

                if self.config.auto_start_breaks {
                    match next_phase {
                        TimerPhase::ShortBreak => self.start_short_break(),
                        TimerPhase::LongBreak => self.start_long_break(),
                        _ => {}
                    }
                } else {
                    self.reset();
                }

                next_phase
            }
            TimerPhase::ShortBreak | TimerPhase::LongBreak => {
                if self.config.auto_start_work {
                    self.start_work();
                } else {
                    self.reset();
                }

                TimerPhase::Work
            }
            TimerPhase::Idle => TimerPhase::Idle,
        }
    }

    fn remaining_duration(&self) -> Duration {
        let Some(started_at) = self.started_at else {
            return self.total_duration;
        };

        let elapsed = if let Some(paused_at) = self.paused_at {
            paused_at.saturating_duration_since(started_at)
        } else {
            started_at.elapsed()
        };

        self.total_duration
            .saturating_sub(elapsed.saturating_sub(self.accumulated_pause))
    }
}

fn display_seconds(duration: Duration) -> u32 {
    let seconds = duration.as_secs().min(u32::MAX as u64) as u32;

    if duration.subsec_nanos() > 0 {
        seconds.saturating_add(1)
    } else {
        seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_work_with_default_duration() {
        let mut timer = TimerEngine::new(TimerConfig::default());

        timer.start_work();

        let snapshot = timer.snapshot();
        assert_eq!(snapshot.phase, TimerPhase::Work);
        assert_eq!(snapshot.remaining_seconds, 25 * 60);
        assert_eq!(snapshot.completed_in_cycle, 0);
    }

    #[test]
    fn skip_work_selects_short_break_before_interval() {
        let mut timer = TimerEngine::new(TimerConfig::default());
        timer.start_work();

        let next = timer.skip();

        assert_eq!(next, TimerPhase::ShortBreak);
        assert_eq!(timer.snapshot().phase, TimerPhase::Idle);
        assert_eq!(timer.snapshot().completed_in_cycle, 1);
    }

    #[test]
    fn fourth_completed_work_selects_long_break() {
        let mut timer = TimerEngine::new(TimerConfig::default());

        let mut next = TimerPhase::Idle;
        for _ in 0..4 {
            timer.start_work();
            next = timer.skip();
        }

        assert_eq!(next, TimerPhase::LongBreak);
        assert_eq!(timer.snapshot().completed_in_cycle, 4);
    }

    #[test]
    fn pause_and_resume_are_stateful() {
        let mut timer = TimerEngine::new(TimerConfig::default());
        timer.start_work();

        assert!(timer.pause());
        assert!(timer.snapshot().is_paused);
        assert!(!timer.pause());
        assert!(timer.resume());
        assert!(!timer.snapshot().is_paused);
        assert!(!timer.resume());
    }
}
