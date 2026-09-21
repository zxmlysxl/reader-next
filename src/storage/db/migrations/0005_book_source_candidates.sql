-- Book source candidates: stores alternate source candidates per book.
-- Key is (user_ns, name, author, origin) — the book being searched FOR, not the found URL.
-- This enables different browsers to see the same cached results.
CREATE TABLE IF NOT EXISTS book_source_candidates (
    user_ns TEXT NOT NULL DEFAULT 'default',
    name TEXT NOT NULL,
    author TEXT NOT NULL,
    origin TEXT NOT NULL,
    book_url TEXT NOT NULL,
    cover_url TEXT,
    intro TEXT,
    kind TEXT,
    latest_chapter_title TEXT,
    update_time INTEGER,
    word_count INTEGER,
    found_at INTEGER NOT NULL,
    PRIMARY KEY (user_ns, name, author, origin)
);

CREATE INDEX IF NOT EXISTS idx_candidates_user_ns ON book_source_candidates(user_ns);
CREATE INDEX IF NOT EXISTS idx_candidates_found_at ON book_source_candidates(found_at);
