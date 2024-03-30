use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: Option<String>,
}

// Define a function to create the routes
pub fn create_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
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

    // Combine the routes
    let routes = hello_route.or(get_user_route);

    routes
}
