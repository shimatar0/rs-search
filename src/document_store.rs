pub struct DocumentStore {}

#[cfg(test)]
mod test {
    use sqlx::{mysql::MySqlPoolOptions, Row};

    fn setup() {
        println!("aiueo");
    }

    #[tokio::test]
    async fn save() {
        let database_url = format!(
            "mysql://{}@{}:{}/{}",
            "root", "localhost", "3306", "tinysearch"
        );
        println!("{}", database_url);
        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .unwrap();

        let rows = sqlx::query(
            r#"
            SELECT * FROM documents
        "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        for row in rows {
            let id: i32 = row.get("id");
            println!("{}", id);
        }
    }
}
