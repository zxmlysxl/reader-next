-- Remote source subscriptions: stores user's subscribed remote source URLs.
-- Persisted on the server side so users see the same subscriptions across devices.
CREATE TABLE IF NOT EXISTS remote_subscriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_ns TEXT NOT NULL DEFAULT 'default',
    url TEXT NOT NULL,
    last_synced_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    UNIQUE(user_ns, url)
);

CREATE INDEX IF NOT EXISTS idx_subs_user_ns ON remote_subscriptions(user_ns);
