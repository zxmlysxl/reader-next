-- Books table: stores each book instance per user with its current source.
-- A book can have multiple entries (different bookUrl/origin combinations),
-- but we maintain a unique constraint on (user_ns, book_url, origin) so
-- each source-instance of a book is one row.
CREATE TABLE IF NOT EXISTS books (
    user_ns TEXT NOT NULL DEFAULT 'default',
    book_url TEXT NOT NULL,
    origin TEXT NOT NULL,
    json TEXT NOT NULL,
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (user_ns, book_url, origin)
);

CREATE INDEX IF NOT EXISTS idx_books_user_ns ON books(user_ns);
CREATE INDEX IF NOT EXISTS idx_books_book_url ON books(book_url);
CREATE INDEX IF NOT EXISTS idx_books_origin ON books(origin);
