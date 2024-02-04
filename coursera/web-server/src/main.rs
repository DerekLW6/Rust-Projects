use warp::Filter;
use std::collections::HashMap;

// Target website while running http://127.0.0.1:8000/hello/user?name=Bob

#[tokio::main]
async fn main() {
    // Define a warp filter that captures the "name" query parameter
    let hello = warp::path!("hello" / "user")
        .and(warp::query::<HashMap<String, String>>())
        .map(|params: HashMap<String, String>| {
            // Create a named variable to extend the lifetime of the temporary value
            let default_name = "User".to_string();
            let name = params.get("name").unwrap_or(&default_name);

            // Respond with a greeting
            warp::reply::html(format!("Hello, {}!", name))
        });

    // Start the warp server
    warp::serve(hello)
        .run(([127, 0, 0, 1], 8000))
        .await;
}
