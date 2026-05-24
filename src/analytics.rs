use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionKind {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRecord {
    pub started_day: String,
    pub started_hour: u8,
    pub kind: SessionKind,
    pub completed: bool,
    pub duration_seconds: u32,
    pub category: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyticsSummary {
    pub completed_work_sessions: u32,
    pub focus_minutes: u32,
    pub completion_rate: f32,
    pub category_minutes: BTreeMap<String, u32>,
    pub best_focus_hour: Option<u8>,
}

pub fn summarize_sessions(records: &[SessionRecord]) -> AnalyticsSummary {
    let mut completed_work_sessions = 0;
    let mut attempted_work_sessions = 0;
    let mut focus_seconds = 0;
    let mut category_minutes = BTreeMap::new();
    let mut hour_seconds = BTreeMap::<u8, u32>::new();

    for record in records {
        if record.kind != SessionKind::Work {
            continue;
        }

        attempted_work_sessions += 1;

        if record.completed {
            completed_work_sessions += 1;
            focus_seconds += record.duration_seconds;

            *category_minutes.entry(record.category.clone()).or_default() +=
                record.duration_seconds / 60;
            *hour_seconds.entry(record.started_hour).or_default() += record.duration_seconds;
        }
    }

    let completion_rate = if attempted_work_sessions == 0 {
        0.0
    } else {
        completed_work_sessions as f32 / attempted_work_sessions as f32
    };

    let best_focus_hour = hour_seconds
        .into_iter()
        .max_by_key(|(_, seconds)| *seconds)
        .map(|(hour, _)| hour);

    AnalyticsSummary {
        completed_work_sessions,
        focus_minutes: focus_seconds / 60,
        completion_rate,
        category_minutes,
        best_focus_hour,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_completed_work_sessions_only() {
        let records = vec![
            SessionRecord {
                started_day: "2026-05-24".to_string(),
                started_hour: 10,
                kind: SessionKind::Work,
                completed: true,
                duration_seconds: 25 * 60,
                category: "General".to_string(),
            },
            SessionRecord {
                started_day: "2026-05-24".to_string(),
                started_hour: 11,
                kind: SessionKind::Work,
                completed: false,
                duration_seconds: 10 * 60,
                category: "General".to_string(),
            },
            SessionRecord {
                started_day: "2026-05-24".to_string(),
                started_hour: 12,
                kind: SessionKind::ShortBreak,
                completed: true,
                duration_seconds: 5 * 60,
                category: "Break".to_string(),
            },
        ];

        let summary = summarize_sessions(&records);

        assert_eq!(summary.completed_work_sessions, 1);
        assert_eq!(summary.focus_minutes, 25);
        assert_eq!(summary.completion_rate, 0.5);
        assert_eq!(summary.category_minutes.get("General"), Some(&25));
        assert_eq!(summary.best_focus_hour, Some(10));
    }
}
