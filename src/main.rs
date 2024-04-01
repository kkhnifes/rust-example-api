use axum::Router;
mod model;
mod repository;
mod person {
    pub use crate::model::{Person,PersonDto};
    pub use crate::repository::PersonRepositoryState;
}
mod controller;
use crate::controller::PersonController;

#[tokio::main]
async fn main() {
    let person_controller =PersonController::default();
    let app: Router = Router::new()
        .merge(person_controller.person());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}


/* 
// get a "ping" endpoint that returns "pong"
fn ping_pong() -> Router {
    async fn pong() -> &'static str {
        "pong"
    }
    Router::new().route("/ping", get(pong))
}

// get a "greet" endpoint that returns a greeting with the name provided
fn greeter() -> Router {
    async fn greet_string(Path(name): Path<String>) -> String {
        format!("Hello {name}!")
    }
    Router::new().route("/greet/:name", get(greet_string))
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct QueryParameters {
    salutation: Option<String>,
    name: Option<String>,
}

fn greeter_query() -> Router {
    async fn greet_string(Query(params): Query<QueryParameters>) -> String {
        let salutation = params.salutation.unwrap_or("Hello".to_string());
        let name = params.name.unwrap_or("World".to_string());
        format!("{salutation} {name}!")
    }
    Router::new().route("/greet", get(greet_string))
}

fn greeter_payload() -> Router {
    async fn greet_string(Json(params): Json<QueryParameters>) -> String {
        let salutation = params.salutation.unwrap_or("Hello".to_string());
        let name = params.name.unwrap_or("World".to_string());
        format!("{salutation} {name}!")
    }
    Router::new().route("/greet", post(greet_string))
}
*/