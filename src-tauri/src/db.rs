use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize)]
pub struct Entry {
    pub id: i64,
    pub text: String,
    pub created_at: NaiveDateTime,
}

pub async fn insert_entry(pool: &Pool<Sqlite>, text: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!("INSERT INTO entries (text) VALUES (?)", text)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}

pub async fn get_all_entries(pool: &Pool<Sqlite>) -> Result<Vec<Entry>, sqlx::Error> {
    let entries = sqlx::query_as!(
        Entry,
        "SELECT id, text, created_at FROM entries ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> Pool<Sqlite> {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create test pool");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    #[tokio::test]
    async fn test_insert_entry_returns_id() {
        let pool = setup_test_db().await;
        let id = insert_entry(&pool, "Test entry").await.unwrap();
        assert!(id > 0);
    }

    #[tokio::test]
    async fn test_get_all_entries_returns_ordered_list() {
        let pool = setup_test_db().await;

        sqlx::query!(
            "INSERT INTO entries (text, created_at) VALUES ('First', '2024-01-01 00:00:00')"
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query!(
            "INSERT INTO entries (text, created_at) VALUES ('Second', '2024-01-01 00:00:01')"
        )
        .execute(&pool)
        .await
        .unwrap();

        let entries = get_all_entries(&pool).await.unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].text, "Second"); // newest first
        assert_eq!(entries[1].text, "First");
    }

    #[tokio::test]
    async fn test_empty_database_returns_empty_vec() {
        let pool = setup_test_db().await;
        let entries = get_all_entries(&pool).await.unwrap();
        assert_eq!(entries.len(), 0);
    }
}
