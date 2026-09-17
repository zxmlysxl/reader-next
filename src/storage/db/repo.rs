use crate::error::error::AppError;
use crate::model::book_source::BookSource;
use crate::util::time::now_ts;
use sqlx::{Row, SqlitePool};

#[derive(Clone)]
pub struct BookSourceRepo {
    pool: SqlitePool,
}

impl BookSourceRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn upsert(
        &self,
        user_ns: &str,
        source: &BookSource,
        json: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO book_sources (user_ns, book_source_url, book_source_name, json, updated_at) VALUES (?1, ?2, ?3, ?4, ?5) \
             ON CONFLICT(user_ns, book_source_url) DO UPDATE SET book_source_name=excluded.book_source_name, json=excluded.json, updated_at=excluded.updated_at"
        )
        .bind(user_ns)
        .bind(&source.book_source_url)
        .bind(&source.book_source_name)
        .bind(json)
        .bind(now_ts())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, user_ns: &str, book_source_url: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM book_sources WHERE user_ns=?1 AND book_source_url=?2")
            .bind(user_ns)
            .bind(book_source_url)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_all(&self, user_ns: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM book_sources WHERE user_ns=?1")
            .bind(user_ns)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get(
        &self,
        user_ns: &str,
        book_source_url: &str,
    ) -> Result<Option<String>, AppError> {
        let row =
            sqlx::query("SELECT json FROM book_sources WHERE user_ns=?1 AND book_source_url=?2")
                .bind(user_ns)
                .bind(book_source_url)
                .fetch_optional(&self.pool)
                .await?;
        Ok(row.map(|r| r.get::<String, _>("json")))
    }

    pub async fn list(&self, user_ns: &str) -> Result<Vec<String>, AppError> {
        let rows =
            sqlx::query("SELECT json FROM book_sources WHERE user_ns=?1 ORDER BY updated_at DESC")
                .bind(user_ns)
                .fetch_all(&self.pool)
                .await?;
        Ok(rows
            .into_iter()
            .map(|r| r.get::<String, _>("json"))
            .collect())
    }

    pub async fn copy_to(&self, from_ns: &str, to_ns: &str) -> Result<i64, AppError> {
        let rows = sqlx::query("SELECT book_source_url, book_source_name, json, updated_at FROM book_sources WHERE user_ns=?1")
            .bind(from_ns)
            .fetch_all(&self.pool)
            .await?;
        let count = rows.len() as i64;
        for row in rows {
            let url: String = row.get("book_source_url");
            let name: String = row.get("book_source_name");
            let json: String = row.get("json");
            let updated_at: i64 = row.get("updated_at");
            sqlx::query(
                "INSERT INTO book_sources (user_ns, book_source_url, book_source_name, json, updated_at) VALUES (?1, ?2, ?3, ?4, ?5) \
                 ON CONFLICT(user_ns, book_source_url) DO UPDATE SET book_source_name=excluded.book_source_name, json=excluded.json, updated_at=excluded.updated_at"
            )
            .bind(to_ns)
            .bind(&url)
            .bind(&name)
            .bind(&json)
            .bind(updated_at)
            .execute(&self.pool)
            .await?;
        }
        Ok(count)
    }
}

use crate::model::book::Book;

/// Repository for books table
#[derive(Clone)]
pub struct BookRepo {
    pool: SqlitePool,
}

impl BookRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Upsert a book (insert or update on conflict)
    pub async fn upsert(&self, user_ns: &str, book: &Book, json: &str) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO books (user_ns, book_url, origin, json, updated_at) VALUES (?1, ?2, ?3, ?4, ?5) \
             ON CONFLICT(user_ns, book_url, origin) DO UPDATE SET json=excluded.json, updated_at=excluded.updated_at"
        )
        .bind(user_ns)
        .bind(&book.book_url)
        .bind(&book.origin)
        .bind(json)
        .bind(now_ts())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Get a specific book by user_ns, book_url, and origin
    pub async fn get(&self, user_ns: &str, book_url: &str, origin: &str) -> Result<Option<String>, AppError> {
        let row = sqlx::query(
            "SELECT json FROM books WHERE user_ns=?1 AND book_url=?2 AND origin=?3"
        )
        .bind(user_ns)
        .bind(book_url)
        .bind(origin)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| r.get::<String, _>("json")))
    }

    /// List all books for a user
    pub async fn list(&self, user_ns: &str) -> Result<Vec<String>, AppError> {
        let rows = sqlx::query("SELECT json FROM books WHERE user_ns=?1 ORDER BY updated_at DESC")
            .bind(user_ns)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(|r| r.get::<String, _>("json")).collect())
    }

    /// Delete a specific book
    pub async fn delete(&self, user_ns: &str, book_url: &str, origin: &str) -> Result<bool, AppError> {
        let result = sqlx::query(
            "DELETE FROM books WHERE user_ns=?1 AND book_url=?2 AND origin=?3"
        )
        .bind(user_ns)
        .bind(book_url)
        .bind(origin)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Delete all books for a user
    pub async fn delete_all(&self, user_ns: &str) -> Result<u64, AppError> {
        let result = sqlx::query("DELETE FROM books WHERE user_ns=?1")
            .bind(user_ns)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Get all books that match a given book_url (across all origins) — used for source-switch
    pub async fn get_by_book_url(&self, user_ns: &str, book_url: &str) -> Result<Vec<String>, AppError> {
        let rows = sqlx::query(
            "SELECT json FROM books WHERE user_ns=?1 AND book_url=?2 ORDER BY updated_at DESC"
        )
        .bind(user_ns)
        .bind(book_url)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.get::<String, _>("json")).collect())
    }

    /// Get all book_url values (deduplicated) for a user — used for getAvailableBookSources
    pub async fn all_book_urls(&self, user_ns: &str) -> Result<Vec<String>, AppError> {
        let rows = sqlx::query(
            "SELECT DISTINCT book_url FROM books WHERE user_ns=?1 ORDER BY updated_at DESC"
        )
        .bind(user_ns)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.get::<String, _>("book_url")).collect())
    }

    /// Migrate from bookshelf.json to database
    pub async fn migrate_from_json(&self, user_ns: &str, books: &[Book]) -> Result<u64, AppError> {
        let mut count = 0u64;
        for book in books {
            let json = serde_json::to_string(book).map_err(|e| AppError::Internal(e.into()))?;
            sqlx::query(
                "INSERT INTO books (user_ns, book_url, origin, json, updated_at) VALUES (?1, ?2, ?3, ?4, ?5) \
                 ON CONFLICT(user_ns, book_url, origin) DO UPDATE SET json=excluded.json, updated_at=excluded.updated_at"
            )
            .bind(user_ns)
            .bind(&book.book_url)
            .bind(&book.origin)
            .bind(&json)
            .bind(now_ts())
            .execute(&self.pool)
            .await?;
            count += 1;
        }
        Ok(count)
    }
}

// ── BookSourceCandidateRepo ──────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BookSourceCandidate {
    pub name: String,
    pub author: String,
    pub book_url: String,
    pub origin: String,
    pub cover_url: Option<String>,
    pub intro: Option<String>,
    pub kind: Option<String>,
    pub latest_chapter_title: Option<String>,
    pub update_time: Option<i64>,
    pub word_count: Option<i64>,
}

pub struct BookSourceCandidateRepo {
    pool: SqlitePool,
}

impl BookSourceCandidateRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Upsert candidates for a book_url — replaces all candidates for that book_url for this user.
    pub async fn upsert_candidates(
        &self,
        user_ns: &str,
        book_url: &str,
        candidates: &[BookSourceCandidate],
    ) -> Result<(), AppError> {
        let mut tx = sqlx::Acquire::begin(&self.pool).await?;
        // Delete existing candidates for this book_url
        sqlx::query("DELETE FROM book_source_candidates WHERE user_ns=?1 AND book_url=?2")
            .bind(user_ns)
            .bind(book_url)
            .execute(&mut *tx)
            .await?;
        let now = now_ts();
        for c in candidates {
            sqlx::query(
                "INSERT INTO book_source_candidates (user_ns, book_url, name, author, origin, \
                 cover_url, intro, kind, latest_chapter_title, update_time, word_count, found_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)"
            )
            .bind(user_ns)
            .bind(&c.book_url)
            .bind(&c.name)
            .bind(&c.author)
            .bind(&c.origin)
            .bind(&c.cover_url)
            .bind(&c.intro)
            .bind(&c.kind)
            .bind(&c.latest_chapter_title)
            .bind(&c.update_time)
            .bind(&c.word_count)
            .bind(now)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Get all candidates for a book_url
    pub async fn get_candidates(
        &self,
        user_ns: &str,
        book_url: &str,
    ) -> Result<Vec<BookSourceCandidate>, AppError> {
        let rows = sqlx::query_as::<_, BookSourceCandidateRow>(
            "SELECT name, author, book_url, origin, cover_url, intro, kind, \
             latest_chapter_title, update_time, word_count \
             FROM book_source_candidates WHERE user_ns=?1 AND book_url=?2 ORDER BY found_at DESC"
        )
        .bind(user_ns)
        .bind(book_url)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Delete all candidates for a book_url
    pub async fn delete_candidates(&self, user_ns: &str, book_url: &str) -> Result<u64, AppError> {
        let result = sqlx::query(
            "DELETE FROM book_source_candidates WHERE user_ns=?1 AND book_url=?2"
        )
        .bind(user_ns)
        .bind(book_url)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct BookSourceCandidateRow {
    name: String,
    author: String,
    book_url: String,
    origin: String,
    cover_url: Option<String>,
    intro: Option<String>,
    kind: Option<String>,
    latest_chapter_title: Option<String>,
    update_time: Option<i64>,
    word_count: Option<i64>,
}

impl From<BookSourceCandidateRow> for BookSourceCandidate {
    fn from(r: BookSourceCandidateRow) -> Self {
        Self {
            name: r.name,
            author: r.author,
            book_url: r.book_url,
            origin: r.origin,
            cover_url: r.cover_url,
            intro: r.intro,
            kind: r.kind,
            latest_chapter_title: r.latest_chapter_title,
            update_time: r.update_time,
            word_count: r.word_count,
        }
    }
}
