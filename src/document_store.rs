use sqlx::{mysql::MySqlPoolOptions, pool, Error, MySql, Pool, Row};

use crate::index::DocumentID;

pub struct DocumentStore {
    pool: Pool<MySql>,
}

impl DocumentStore {
    pub async fn new() -> Self {
        let database_url = format!(
            "mysql://{}@{}:{}/{}",
            "root", "localhost", "3306", "tinysearch"
        );
        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .unwrap();
        DocumentStore { pool }
    }

    pub async fn save(&self, title: String) -> DocumentID {
        sqlx::query(
            r#"
            INSERT INTO documents (document_title) VALUES (?)
        "#,
        )
        .bind(title)
        .execute(&self.pool)
        .await
        .unwrap();

        let last_insert_id = sqlx::query_as::<_, (u32,)>(
            r#"
            SELECT LAST_INSERT_ID();
        "#,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
        .0;

        return last_insert_id as DocumentID;
    }

    pub async fn get_rows(&self) {
        let query: String = r#"
            SELECT * FROM documents
        "#
        .to_owned();
        let rows = sqlx::query(&query).fetch_all(&self.pool).await.unwrap();
        for row in rows {
            let id: u32 = row.get("document_id");
            println!("{}", id);
        }
    }

    pub async fn truncate_table(&self) -> Result<(), Error> {
        sqlx::query(&format!("TRUNCATE TABLE documents"))
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
