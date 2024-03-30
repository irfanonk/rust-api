mod routes;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio_postgres::{Client, Error, NoTls};

async fn connect() -> Result<Arc<Client>, Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Arc::new(client))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let client = match connect().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("connection error: {}", e);
            return;
        }
    };
    // Import the routes from routes.rs
    let routes = routes::create_routes(client);

    // Start the Warp server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
