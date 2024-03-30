mod routes;

#[tokio::main]
async fn main() {
    // Import the routes from routes.rs
    let routes = routes::create_routes();

    // Start the Warp server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
