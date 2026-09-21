-- Fix book_source_candidates key: change from (user_ns, book_url, origin) to
-- (user_ns, name, author, origin). This matches how the search stores candidates
-- (by the searched book's name/author) with how available-source reads them.

-- Add unique constraint on the correct key (preserves existing data, new inserts use this)
CREATE UNIQUE INDEX IF NOT EXISTS idx_candidates_lookup ON book_source_candidates(user_ns, name, author, origin);

-- book_url column is kept as payload data, not part of the lookup key.
-- Old PK index no longer needed; drop it to avoid confusion.
DROP INDEX IF EXISTS idx_candidates_book_url;
