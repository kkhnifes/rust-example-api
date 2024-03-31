use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app: Router = Router::new().nest("/test", ping_pong());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn ping_pong() -> Router {
    async fn hello() -> &'static str {
        "pong"
    }
    Router::new().route("/ping", get(hello))
}
