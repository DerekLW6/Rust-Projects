use warp::Filter;
use std::collections::HashMap;

// Rust Candle Location - https://github.com/huggingface/candle?tab=readme-ov-file

#[tokio::main]
async fn main() {
    // Define a warp filter for the root path
    let root = warp::path::end().map(|| warp::reply::html(html_form()));

    // Define a warp filter for the form submission
    let submit_form = warp::path!("submit" / "user")
        .and(warp::query::<HashMap<String, String>>())
        .map(|params: HashMap<String, String>| {
            // Get the user's query from the form submission
            let user_query = params.get("name").cloned().unwrap_or_default();

            // Response
            warp::reply::html(format!("{}", user_query))
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
            <title>Rust Web Server For LLMs</title>
        </head>
        <body>
            <h1>Welcome to the Rust Web Server For LLMs</h1>
            <form action="/submit/user" method="get">
                <label for="name">Enter your Query:</label>
                <!-- Use the 'textarea' tag for a larger text box -->
                <textarea id="name" name="name" rows="4" cols="50" required></textarea>
                <br>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>
    "#
    .to_string()
}
