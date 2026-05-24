CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'General',
    priority TEXT NOT NULL CHECK (priority IN ('low', 'medium', 'high')),
    status TEXT NOT NULL CHECK (status IN ('active', 'completed')),
    due_date TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT,
    pomodoros_spent INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_tasks_status_priority
ON tasks(status, priority, created_at);

CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL CHECK (kind IN ('work', 'short_break', 'long_break')),
    category TEXT NOT NULL DEFAULT 'General',
    started_at TEXT NOT NULL,
    completed_at TEXT,
    duration_seconds INTEGER NOT NULL,
    completed INTEGER NOT NULL CHECK (completed IN (0, 1))
);

CREATE INDEX IF NOT EXISTS idx_sessions_started_at
ON sessions(started_at);
