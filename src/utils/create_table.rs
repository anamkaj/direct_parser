use sqlx::{Pool, Postgres};

pub async fn create_table(pool: &Pool<Postgres>) -> Result<String, Box<dyn std::error::Error>> {
    let check_table: &str = "SELECT EXISTS (
    SELECT 1
    FROM pg_tables
    WHERE schemaname = 'public'
    AND tablename = 'status_table'
);";

    let row: (bool,) = sqlx::query_as(&check_table).fetch_one(pool).await?;
    let table_exists = row.0;

    if table_exists {
        return Ok("Table already exists".to_string());
    }

    if !table_exists {
        let status_table: &str = "
            CREATE TABLE public.status_table (
            id bigserial NOT NULL,
            date timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
            status varchar NOT NULL,
            CONSTRAINT status_table_pkey PRIMARY KEY (id));";

        sqlx::query(&status_table)
            .execute(pool)
            .await
            .expect("Error creating table");
    }

    Ok("Table created successfully!".to_string())
}
