use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::{Client, Error};
use warp::{Filter, Rejection, Reply};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Product {
    pub name: String,
    pub manufacturer: String,
    pub description: String,
    pub added: String,
    pub item_type: String,
    pub tags: String,
    pub slug: String,
}

// Function to read products from the database
async fn read_products(client: &Client) -> Result<Vec<Product>, Error> {
    let rows = client.query("SELECT * FROM product", &[]).await?;

    let mut products = Vec::new();

    for row in rows {
        let name: String = row.get("name");
        let description: String = row.get("description");
        let manufacturer: String = row.get("manufacturer");
        let added: String = row.get("added");
        let item_type: String = row.get("itemType");
        let tags: String = row.get("tags");
        let slug: String = row.get("slug");

        let product = Product {
            name,
            description,
            manufacturer,
            added,
            item_type,
            tags,
            slug,
        };
        products.push(product);
    }

    Ok(products)
}

pub async fn get_products_handler(client: Arc<Client>) -> Result<impl Reply, Rejection> {
    let products = read_products(&client).await.map_err(|e| {
        eprintln!("Database error: {}", e);
        warp::reject::reject()
    })?;

    Ok(warp::reply::json(&products))
}
// Helper function to inject the database client into warp filter chain
fn with_db(
    client: Arc<Client>,
) -> impl Filter<Extract = (Arc<Client>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}
// Define a function to create the routes
pub fn create_routes(
    client: Arc<Client>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // Define a route for GET requests to "/"
    let hello_route = warp::path::end().map(|| warp::reply::html("Hello, world!"));

    // Define a route for GET requests to "/user"
    let get_user_route = warp::path("user").and(warp::get()).map(|| {
        // Create a sample User object (replace this with your actual logic)
        let user = User {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: None,
        };

        // Serialize the User object to JSON and return it
        warp::reply::json(&user)
    });
    let get_products_route = warp::path("products")
        .and(warp::get())
        .and(with_db(client.clone()))
        .and_then(get_products_handler);

    // Combine the routes
    let routes = hello_route.or(get_user_route).or(get_products_route);

    routes
}
