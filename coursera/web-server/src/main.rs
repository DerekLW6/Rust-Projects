use warp::Filter;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Define a warp filter for the root path
    let root = warp::path::end().map(|| warp::reply::html(html_form()));

    // Define a warp filter for the form submission
    let submit_form = warp::path!("submit" / "user")
        .and(warp::query::<HashMap<String, String>>())
        .map(|params: HashMap<String, String>| {
            // Create a named variable to extend the lifetime of the temporary value
            let default_name = "User".to_string();
            let name = params.get("name").unwrap_or(&default_name);

            // Respond with a greeting
            warp::reply::html(format!("Hello, {}!", name))
        });

    // Combine the root and form submission filters
    let routes = root.or(submit_form);

    // Start the warp server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}

// Function to generate the HTML form
fn html_form() -> String {
    r#"
    <html>
        <head>
            <title>Rust Web Server</title>
        </head>
        <body>
            <h1>Welcome to the Rust Web Server</h1>
            <form action="/submit/user" method="get">
                <label for="name">Enter your name:</label>
                <input type="text" id="name" name="name" required>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>
    "#
    .to_string()
}
