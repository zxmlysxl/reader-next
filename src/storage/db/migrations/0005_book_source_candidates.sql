-- Book source candidates: stores the search results for finding book sources.
-- When user clicks "search sources" for a book, results are saved here.
-- This enables:
-- 1. Different browsers see the same cached results (no re-search)
-- 2. Source switching remembers what sources were found for this book
CREATE TABLE IF NOT EXISTS book_source_candidates (
    user_ns TEXT NOT NULL DEFAULT 'default',
    book_url TEXT NOT NULL,
    name TEXT NOT NULL,
    author TEXT NOT NULL,
    origin TEXT NOT NULL,
    cover_url TEXT,
    intro TEXT,
    kind TEXT,
    latest_chapter_title TEXT,
    update_time INTEGER,
    word_count INTEGER,
    found_at INTEGER NOT NULL,
    PRIMARY KEY (user_ns, book_url, origin)
);

CREATE INDEX IF NOT EXISTS idx_candidates_user_ns ON book_source_candidates(user_ns);
CREATE INDEX IF NOT EXISTS idx_candidates_book_url ON book_source_candidates(book_url);
CREATE INDEX IF NOT EXISTS idx_candidates_found_at ON book_source_candidates(found_at);
