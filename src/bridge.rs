use crate::analytics::{summarize_sessions, SessionKind, SessionRecord};
use crate::theme::{next_accent, next_theme};
use crate::timer_engine::{TimerConfig, TimerEngine, TimerPhase, TimerSnapshot};
use chrono::{Datelike, Local, Timelike};
use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

pub struct PolindoraControllerRust {
    phase: QString,
    remaining_seconds: i32,
    total_seconds: i32,
    progress: f32,
    is_running: bool,
    is_paused: bool,
    strict_mode: bool,
    completed_in_cycle: i32,
    sessions_today: i32,
    focus_minutes_today: i32,
    active_category: QString,
    quote_text: QString,
    work_duration_minutes: i32,
    short_break_minutes: i32,
    long_break_minutes: i32,
    long_break_interval: i32,
    auto_start_breaks: bool,
    auto_start_work: bool,
    bar_width: i32,
    bar_height: i32,
    bar_radius: i32,
    glow_intensity: i32,
    color_saturation_percent: i32,
    analytics_completed_sessions: i32,
    analytics_focus_minutes: i32,
    analytics_completion_rate_percent: i32,
    analytics_best_focus_time: QString,
    analytics_trend_text: QString,
    analytics_history_text: QString,
    task_total_count: i32,
    task_active_count: i32,
    task_completed_count: i32,
    task_filter: QString,
    task_list_text: QString,
    active_theme_name: QString,
    active_theme_description: QString,
    active_accent_name: QString,
    active_accent_color: QString,
    tasks: Vec<ControllerTask>,
    session_history: Vec<SessionRecord>,
    timer: TimerEngine,
}

impl Default for PolindoraControllerRust {
    fn default() -> Self {
        let timer = TimerEngine::new(TimerConfig::default());
        let snapshot = timer.snapshot();

        Self {
            phase: phase_qstring(snapshot.phase),
            remaining_seconds: snapshot.remaining_seconds as i32,
            total_seconds: snapshot.total_seconds as i32,
            progress: snapshot.progress,
            is_running: false,
            is_paused: false,
            strict_mode: snapshot.strict_mode,
            completed_in_cycle: snapshot.completed_in_cycle as i32,
            sessions_today: 0,
            focus_minutes_today: 0,
            active_category: QString::from("General"),
            quote_text: QString::from(
                "All power is within you; you can do anything and everything. Believe in that. - Swami Vivekananda",
            ),
            work_duration_minutes: (TimerConfig::default().work_seconds / 60) as i32,
            short_break_minutes: (TimerConfig::default().short_break_seconds / 60) as i32,
            long_break_minutes: (TimerConfig::default().long_break_seconds / 60) as i32,
            long_break_interval: TimerConfig::default().long_break_interval as i32,
            auto_start_breaks: TimerConfig::default().auto_start_breaks,
            auto_start_work: TimerConfig::default().auto_start_work,
            bar_width: 140,
            bar_height: 4,
            bar_radius: 99,
            glow_intensity: 3,
            color_saturation_percent: 100,
            analytics_completed_sessions: 0,
            analytics_focus_minutes: 0,
            analytics_completion_rate_percent: 0,
            analytics_best_focus_time: QString::from("No focus sessions yet"),
            analytics_trend_text: QString::from("Complete a focus session to start tracking"),
            analytics_history_text: QString::from("No sessions recorded"),
            task_total_count: 0,
            task_active_count: 0,
            task_completed_count: 0,
            task_filter: QString::from("All"),
            task_list_text: QString::from("No active tasks"),
            active_theme_name: QString::from("Polindora Dark"),
            active_theme_description: QString::from("Deep glass surfaces with electric blue glow"),
            active_accent_name: QString::from("Blue"),
            active_accent_color: QString::from("#58a6ff"),
            tasks: Vec::new(),
            session_history: Vec::new(),
            timer,
        }
    }
}

#[cxx_qt::bridge]
pub mod qobject {
    use core::pin::Pin;

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, phase)]
        #[qproperty(i32, remaining_seconds, cxx_name = "remainingSeconds")]
        #[qproperty(i32, total_seconds, cxx_name = "totalSeconds")]
        #[qproperty(f32, progress)]
        #[qproperty(bool, is_running, cxx_name = "isRunning")]
        #[qproperty(bool, is_paused, cxx_name = "isPaused")]
        #[qproperty(bool, strict_mode, cxx_name = "strictMode")]
        #[qproperty(i32, completed_in_cycle, cxx_name = "completedInCycle")]
        #[qproperty(i32, sessions_today, cxx_name = "sessionsToday")]
        #[qproperty(i32, focus_minutes_today, cxx_name = "focusMinutesToday")]
        #[qproperty(QString, active_category, cxx_name = "activeCategory")]
        #[qproperty(QString, quote_text, cxx_name = "quoteText")]
        #[qproperty(i32, work_duration_minutes, cxx_name = "workDurationMinutes")]
        #[qproperty(i32, short_break_minutes, cxx_name = "shortBreakMinutes")]
        #[qproperty(i32, long_break_minutes, cxx_name = "longBreakMinutes")]
        #[qproperty(i32, long_break_interval, cxx_name = "longBreakInterval")]
        #[qproperty(bool, auto_start_breaks, cxx_name = "autoStartBreaks")]
        #[qproperty(bool, auto_start_work, cxx_name = "autoStartWork")]
        #[qproperty(i32, bar_width, cxx_name = "barWidth")]
        #[qproperty(i32, bar_height, cxx_name = "barHeight")]
        #[qproperty(i32, bar_radius, cxx_name = "barRadius")]
        #[qproperty(i32, glow_intensity, cxx_name = "glowIntensity")]
        #[qproperty(i32, color_saturation_percent, cxx_name = "colorSaturationPercent")]
        #[qproperty(i32, analytics_completed_sessions, cxx_name = "analyticsCompletedSessions")]
        #[qproperty(i32, analytics_focus_minutes, cxx_name = "analyticsFocusMinutes")]
        #[qproperty(i32, analytics_completion_rate_percent, cxx_name = "analyticsCompletionRatePercent")]
        #[qproperty(QString, analytics_best_focus_time, cxx_name = "analyticsBestFocusTime")]
        #[qproperty(QString, analytics_trend_text, cxx_name = "analyticsTrendText")]
        #[qproperty(QString, analytics_history_text, cxx_name = "analyticsHistoryText")]
        #[qproperty(i32, task_total_count, cxx_name = "taskTotalCount")]
        #[qproperty(i32, task_active_count, cxx_name = "taskActiveCount")]
        #[qproperty(i32, task_completed_count, cxx_name = "taskCompletedCount")]
        #[qproperty(QString, task_filter, cxx_name = "taskFilter")]
        #[qproperty(QString, task_list_text, cxx_name = "taskListText")]
        #[qproperty(QString, active_theme_name, cxx_name = "activeThemeName")]
        #[qproperty(QString, active_theme_description, cxx_name = "activeThemeDescription")]
        #[qproperty(QString, active_accent_name, cxx_name = "activeAccentName")]
        #[qproperty(QString, active_accent_color, cxx_name = "activeAccentColor")]
        type PolindoraController = super::PolindoraControllerRust;

        #[qinvokable]
        #[cxx_name = "startWork"]
        fn start_work(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "pauseTimer"]
        fn pause_timer(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "resumeTimer"]
        fn resume_timer(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "skipTimer"]
        fn skip_timer(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "resetTimer"]
        fn reset_timer(self: Pin<&mut PolindoraController>);

        #[qsignal]
        #[cxx_name = "timerCompleted"]
        fn timer_completed(self: Pin<&mut PolindoraController>, next_phase: QString);

        #[qinvokable]
        #[cxx_name = "incrementWorkDuration"]
        fn increment_work_duration(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "decrementWorkDuration"]
        fn decrement_work_duration(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "incrementShortBreak"]
        fn increment_short_break(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "decrementShortBreak"]
        fn decrement_short_break(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "incrementLongBreak"]
        fn increment_long_break(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "decrementLongBreak"]
        fn decrement_long_break(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "incrementLongBreakInterval"]
        fn increment_long_break_interval(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "decrementLongBreakInterval"]
        fn decrement_long_break_interval(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "toggleStrictMode"]
        fn toggle_strict_mode(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "toggleAutoStartBreaks"]
        fn toggle_auto_start_breaks(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "toggleAutoStartWork"]
        fn toggle_auto_start_work(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "incrementBarWidth"]
        fn increment_bar_width(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "decrementBarWidth"]
        fn decrement_bar_width(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "incrementBarHeight"]
        fn increment_bar_height(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "decrementBarHeight"]
        fn decrement_bar_height(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "incrementBarRadius"]
        fn increment_bar_radius(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "decrementBarRadius"]
        fn decrement_bar_radius(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "incrementGlowIntensity"]
        fn increment_glow_intensity(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "decrementGlowIntensity"]
        fn decrement_glow_intensity(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "incrementColorSaturation"]
        fn increment_color_saturation(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "decrementColorSaturation"]
        fn decrement_color_saturation(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "createTask"]
        fn create_task(self: Pin<&mut PolindoraController>, title: &QString);

        #[qinvokable]
        #[cxx_name = "completeTopTask"]
        fn complete_top_task(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "cycleTaskFilter"]
        fn cycle_task_filter(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "cycleTheme"]
        fn cycle_theme(self: Pin<&mut PolindoraController>);

        #[qinvokable]
        #[cxx_name = "cycleAccent"]
        fn cycle_accent(self: Pin<&mut PolindoraController>);
    }
}

impl qobject::PolindoraController {
    pub fn start_work(mut self: Pin<&mut Self>) {
        let snapshot = {
            let mut rust = self.as_mut().rust_mut();
            rust.as_mut().timer.start_work();
            rust.as_ref().timer.snapshot()
        };

        self.apply_snapshot(snapshot);
    }

    pub fn pause_timer(mut self: Pin<&mut Self>) {
        let snapshot = {
            let mut rust = self.as_mut().rust_mut();
            rust.as_mut().timer.pause();
            rust.as_ref().timer.snapshot()
        };

        self.apply_snapshot(snapshot);
    }

    pub fn resume_timer(mut self: Pin<&mut Self>) {
        let snapshot = {
            let mut rust = self.as_mut().rust_mut();
            rust.as_mut().timer.resume();
            rust.as_ref().timer.snapshot()
        };

        self.apply_snapshot(snapshot);
    }

    pub fn skip_timer(mut self: Pin<&mut Self>) {
        let (next_phase, snapshot, should_record_work) = {
            let mut rust = self.as_mut().rust_mut();
            let previous = rust.as_ref().timer.snapshot();
            let next_phase = rust.as_mut().timer.skip();
            let snapshot = rust.as_ref().timer.snapshot();
            (next_phase, snapshot, previous.phase == TimerPhase::Work)
        };

        if should_record_work {
            self.as_mut().record_completed_work_session();
        }

        self.as_mut().apply_snapshot(snapshot);
        self.timer_completed(phase_qstring(next_phase));
    }

    pub fn reset_timer(mut self: Pin<&mut Self>) {
        let snapshot = {
            let mut rust = self.as_mut().rust_mut();
            rust.as_mut().timer.reset();
            rust.as_ref().timer.snapshot()
        };

        self.apply_snapshot(snapshot);
    }

    pub fn increment_work_duration(self: Pin<&mut Self>) {
        self.adjust_timer_config(1, 0, 0, 0);
    }

    pub fn decrement_work_duration(self: Pin<&mut Self>) {
        self.adjust_timer_config(-1, 0, 0, 0);
    }

    pub fn increment_short_break(self: Pin<&mut Self>) {
        self.adjust_timer_config(0, 1, 0, 0);
    }

    pub fn decrement_short_break(self: Pin<&mut Self>) {
        self.adjust_timer_config(0, -1, 0, 0);
    }

    pub fn increment_long_break(self: Pin<&mut Self>) {
        self.adjust_timer_config(0, 0, 1, 0);
    }

    pub fn decrement_long_break(self: Pin<&mut Self>) {
        self.adjust_timer_config(0, 0, -1, 0);
    }

    pub fn increment_long_break_interval(self: Pin<&mut Self>) {
        self.adjust_timer_config(0, 0, 0, 1);
    }

    pub fn decrement_long_break_interval(self: Pin<&mut Self>) {
        self.adjust_timer_config(0, 0, 0, -1);
    }

    pub fn toggle_strict_mode(self: Pin<&mut Self>) {
        self.toggle_behavior_flag(BehaviorFlag::StrictMode);
    }

    pub fn toggle_auto_start_breaks(self: Pin<&mut Self>) {
        self.toggle_behavior_flag(BehaviorFlag::AutoStartBreaks);
    }

    pub fn toggle_auto_start_work(self: Pin<&mut Self>) {
        self.toggle_behavior_flag(BehaviorFlag::AutoStartWork);
    }

    pub fn increment_bar_width(mut self: Pin<&mut Self>) {
        let value = clamp_value(*self.as_ref().bar_width(), 10, 60, 300);
        self.as_mut().set_bar_width(value);
    }

    pub fn decrement_bar_width(mut self: Pin<&mut Self>) {
        let value = clamp_value(*self.as_ref().bar_width(), -10, 60, 300);
        self.as_mut().set_bar_width(value);
    }

    pub fn increment_bar_height(mut self: Pin<&mut Self>) {
        let value = clamp_value(*self.as_ref().bar_height(), 1, 2, 12);
        self.as_mut().set_bar_height(value);
    }

    pub fn decrement_bar_height(mut self: Pin<&mut Self>) {
        let value = clamp_value(*self.as_ref().bar_height(), -1, 2, 12);
        self.as_mut().set_bar_height(value);
    }

    pub fn increment_bar_radius(mut self: Pin<&mut Self>) {
        let value = clamp_value(*self.as_ref().bar_radius(), 5, 0, 99);
        self.as_mut().set_bar_radius(value);
    }

    pub fn decrement_bar_radius(mut self: Pin<&mut Self>) {
        let value = clamp_value(*self.as_ref().bar_radius(), -5, 0, 99);
        self.as_mut().set_bar_radius(value);
    }

    pub fn increment_glow_intensity(mut self: Pin<&mut Self>) {
        let value = clamp_value(*self.as_ref().glow_intensity(), 1, 0, 20);
        self.as_mut().set_glow_intensity(value);
    }

    pub fn decrement_glow_intensity(mut self: Pin<&mut Self>) {
        let value = clamp_value(*self.as_ref().glow_intensity(), -1, 0, 20);
        self.as_mut().set_glow_intensity(value);
    }

    pub fn increment_color_saturation(mut self: Pin<&mut Self>) {
        let value = clamp_value(*self.as_ref().color_saturation_percent(), 10, 0, 200);
        self.as_mut().set_color_saturation_percent(value);
    }

    pub fn decrement_color_saturation(mut self: Pin<&mut Self>) {
        let value = clamp_value(*self.as_ref().color_saturation_percent(), -10, 0, 200);
        self.as_mut().set_color_saturation_percent(value);
    }

    pub fn create_task(mut self: Pin<&mut Self>, title: &QString) {
        let trimmed = title.to_string().trim().to_string();
        if trimmed.is_empty() {
            return;
        }

        {
            let mut rust = self.as_mut().rust_mut();
            let id = rust.as_ref().tasks.len() as u32 + 1;
            rust.as_mut().tasks.push(ControllerTask {
                id,
                title: trimmed,
                category: "General".to_string(),
                priority: TaskPriorityView::Medium,
                completed: false,
                pomodoros_spent: 0,
            });
        }

        self.refresh_task_properties();
    }

    pub fn complete_top_task(mut self: Pin<&mut Self>) {
        {
            let mut rust = self.as_mut().rust_mut();
            if let Some(task) = rust.as_mut().tasks.iter_mut().find(|task| !task.completed) {
                task.completed = true;
            }
        }

        self.refresh_task_properties();
    }

    pub fn cycle_task_filter(mut self: Pin<&mut Self>) {
        {
            let mut rust = self.as_mut().rust_mut();
            let current = rust.as_ref().task_filter.to_string();
            let next = match current.as_str() {
                "All" => "Active",
                "Active" => "Completed",
                _ => "All",
            };
            rust.as_mut().task_filter = QString::from(next);
        }

        self.refresh_task_properties();
    }

    pub fn cycle_theme(mut self: Pin<&mut Self>) {
        let current = self.as_ref().active_theme_name().to_string();
        let next = next_theme(&current);

        self.as_mut().set_active_theme_name(QString::from(next.name));
        self.as_mut()
            .set_active_theme_description(QString::from(next.description));
    }

    pub fn cycle_accent(mut self: Pin<&mut Self>) {
        let current = self.as_ref().active_accent_name().to_string();
        let next = next_accent(&current);

        self.as_mut().set_active_accent_name(QString::from(next.name));
        self.as_mut()
            .set_active_accent_color(QString::from(next.color));
    }

    fn apply_snapshot(mut self: Pin<&mut Self>, snapshot: TimerSnapshot) {
        self.as_mut().set_phase(phase_qstring(snapshot.phase));
        self.as_mut()
            .set_remaining_seconds(snapshot.remaining_seconds as i32);
        self.as_mut().set_total_seconds(snapshot.total_seconds as i32);
        self.as_mut().set_progress(snapshot.progress);
        self.as_mut()
            .set_is_running(snapshot.phase != TimerPhase::Idle && !snapshot.is_paused);
        self.as_mut().set_is_paused(snapshot.is_paused);
        self.as_mut().set_strict_mode(snapshot.strict_mode);
        self.as_mut()
            .set_completed_in_cycle(snapshot.completed_in_cycle as i32);

        self.refresh_analytics_properties();
    }

    fn adjust_timer_config(
        mut self: Pin<&mut Self>,
        work_delta: i32,
        short_delta: i32,
        long_delta: i32,
        interval_delta: i32,
    ) {
        let (config, snapshot) = {
            let mut rust = self.as_mut().rust_mut();
            let current = rust.as_ref().timer.config();
            let config = TimerConfig {
                work_seconds: clamp_minutes(current.work_seconds, work_delta, 1, 60) * 60,
                short_break_seconds: clamp_minutes(current.short_break_seconds, short_delta, 1, 30)
                    * 60,
                long_break_seconds: clamp_minutes(current.long_break_seconds, long_delta, 1, 60)
                    * 60,
                long_break_interval: clamp_value(
                    current.long_break_interval as i32,
                    interval_delta,
                    2,
                    8,
                ) as u32,
                ..current
            };
            rust.as_mut().timer.set_config(config);
            (config, rust.as_ref().timer.snapshot())
        };

        self.as_mut()
            .set_work_duration_minutes((config.work_seconds / 60) as i32);
        self.as_mut()
            .set_short_break_minutes((config.short_break_seconds / 60) as i32);
        self.as_mut()
            .set_long_break_minutes((config.long_break_seconds / 60) as i32);
        self.as_mut()
            .set_long_break_interval(config.long_break_interval as i32);
        self.apply_snapshot(snapshot);
    }

    fn toggle_behavior_flag(mut self: Pin<&mut Self>, flag: BehaviorFlag) {
        let (config, snapshot) = {
            let mut rust = self.as_mut().rust_mut();
            let current = rust.as_ref().timer.config();
            let config = match flag {
                BehaviorFlag::StrictMode => TimerConfig {
                    strict_mode: !current.strict_mode,
                    ..current
                },
                BehaviorFlag::AutoStartBreaks => TimerConfig {
                    auto_start_breaks: !current.auto_start_breaks,
                    ..current
                },
                BehaviorFlag::AutoStartWork => TimerConfig {
                    auto_start_work: !current.auto_start_work,
                    ..current
                },
            };
            rust.as_mut().timer.set_config(config);
            (config, rust.as_ref().timer.snapshot())
        };

        self.as_mut().set_strict_mode(config.strict_mode);
        self.as_mut().set_auto_start_breaks(config.auto_start_breaks);
        self.as_mut().set_auto_start_work(config.auto_start_work);
        self.apply_snapshot(snapshot);
    }

    fn record_completed_work_session(mut self: Pin<&mut Self>) {
        let now = Local::now();
        let record = {
            let rust = self.as_mut().rust_mut();
            let config = rust.as_ref().timer.config();

            SessionRecord {
                started_day: format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day()),
                started_hour: now.hour() as u8,
                kind: SessionKind::Work,
                completed: true,
                duration_seconds: config.work_seconds,
                category: rust.as_ref().active_category.to_string(),
            }
        };

        self.as_mut().rust_mut().session_history.push(record);
        self.refresh_analytics_properties();
    }

    fn refresh_analytics_properties(mut self: Pin<&mut Self>) {
        let (summary, history_text) = {
            let rust = self.as_ref().rust();
            (
                summarize_sessions(&rust.session_history),
                format_history_text(&rust.session_history),
            )
        };

        self.as_mut()
            .set_sessions_today(summary.completed_work_sessions as i32);
        self.as_mut()
            .set_focus_minutes_today(summary.focus_minutes as i32);
        self.as_mut()
            .set_analytics_completed_sessions(summary.completed_work_sessions as i32);
        self.as_mut()
            .set_analytics_focus_minutes(summary.focus_minutes as i32);
        self.as_mut()
            .set_analytics_completion_rate_percent((summary.completion_rate * 100.0).round() as i32);
        self.as_mut()
            .set_analytics_best_focus_time(best_focus_time_text(summary.best_focus_hour));
        self.as_mut()
            .set_analytics_trend_text(trend_text(summary.focus_minutes));
        self.as_mut()
            .set_analytics_history_text(QString::from(history_text));
    }

    fn refresh_task_properties(mut self: Pin<&mut Self>) {
        let (total, active, completed, filter, text) = {
            let rust = self.as_ref().rust();
            let total = rust.tasks.len() as i32;
            let active = rust.tasks.iter().filter(|task| !task.completed).count() as i32;
            let completed = total - active;
            let filter = rust.task_filter.clone();
            let text = format_task_list(&rust.tasks, &filter.to_string());
            (total, active, completed, filter, text)
        };

        self.as_mut().set_task_total_count(total);
        self.as_mut().set_task_active_count(active);
        self.as_mut().set_task_completed_count(completed);
        self.as_mut().set_task_filter(filter);
        self.as_mut().set_task_list_text(QString::from(text));
    }
}

enum BehaviorFlag {
    StrictMode,
    AutoStartBreaks,
    AutoStartWork,
}

struct ControllerTask {
    id: u32,
    title: String,
    category: String,
    priority: TaskPriorityView,
    completed: bool,
    pomodoros_spent: u32,
}

enum TaskPriorityView {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppBridgeState {
    pub phase: String,
    pub remaining_seconds: u32,
    pub total_seconds: u32,
    pub progress: f32,
    pub is_running: bool,
    pub is_paused: bool,
    pub strict_mode: bool,
    pub completed_in_cycle: u32,
}

impl AppBridgeState {
    pub fn from_timer(timer: &TimerEngine) -> Self {
        let snapshot = timer.snapshot();

        Self {
            phase: phase_name(snapshot.phase).to_string(),
            remaining_seconds: snapshot.remaining_seconds,
            total_seconds: snapshot.total_seconds,
            progress: snapshot.progress,
            is_running: snapshot.phase != TimerPhase::Idle && !snapshot.is_paused,
            is_paused: snapshot.is_paused,
            strict_mode: snapshot.strict_mode,
            completed_in_cycle: snapshot.completed_in_cycle,
        }
    }
}

fn phase_name(phase: TimerPhase) -> &'static str {
    match phase {
        TimerPhase::Idle => "idle",
        TimerPhase::Work => "work",
        TimerPhase::ShortBreak => "short_break",
        TimerPhase::LongBreak => "long_break",
    }
}

fn phase_qstring(phase: TimerPhase) -> QString {
    QString::from(phase_name(phase))
}

fn clamp_minutes(seconds: u32, delta: i32, min: i32, max: i32) -> u32 {
    clamp_value(seconds as i32 / 60, delta, min, max) as u32
}

fn clamp_value(value: i32, delta: i32, min: i32, max: i32) -> i32 {
    (value + delta).clamp(min, max)
}

fn best_focus_time_text(hour: Option<u8>) -> QString {
    let Some(hour) = hour else {
        return QString::from("No focus sessions yet");
    };

    QString::from(format!("{:02}:00 - {:02}:00", hour, (hour + 2) % 24))
}

fn trend_text(focus_minutes: u32) -> QString {
    if focus_minutes == 0 {
        QString::from("Complete a focus session to start tracking")
    } else {
        QString::from(format!("{}m focused from real session history", focus_minutes))
    }
}

fn format_history_text(history: &[SessionRecord]) -> String {
    if history.is_empty() {
        return "No sessions recorded".to_string();
    }

    history
        .iter()
        .rev()
        .take(4)
        .map(|record| {
            let status = if record.completed { "completed" } else { "stopped" };
            format!(
                "{} focus - {}m - {}",
                record.category,
                record.duration_seconds / 60,
                status
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_task_list(tasks: &[ControllerTask], filter: &str) -> String {
    let visible = tasks.iter().filter(|task| match filter {
        "Active" => !task.completed,
        "Completed" => task.completed,
        _ => true,
    });

    let lines = visible
        .take(5)
        .map(|task| {
            let status = if task.completed { "done" } else { "active" };
            format!(
                "#{} {} - {} - {} - {} pomodoros",
                task.id,
                task.title,
                priority_label(&task.priority),
                status,
                task.pomodoros_spent
            )
        })
        .collect::<Vec<_>>();

    if lines.is_empty() {
        match filter {
            "Completed" => "No completed tasks".to_string(),
            "Active" => "No active tasks".to_string(),
            _ => "No tasks yet".to_string(),
        }
    } else {
        lines.join("\n")
    }
}

fn priority_label(priority: &TaskPriorityView) -> &'static str {
    match priority {
        TaskPriorityView::Low => "low",
        TaskPriorityView::Medium => "medium",
        TaskPriorityView::High => "high",
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::timer_engine::TimerConfig;

    #[test]
    fn bridge_snapshot_uses_qml_friendly_phase_names() {
        let mut timer = TimerEngine::new(TimerConfig::default());
        timer.start_work();

        let state = AppBridgeState::from_timer(&timer);

        assert_eq!(state.phase, "work");
        assert!(state.is_running);
        assert_eq!(state.remaining_seconds, 25 * 60);
    }
}
