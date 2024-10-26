pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect() -> Result<Self, Error> {
        let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }
}
