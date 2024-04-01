use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use crate::person::*;

#[derive(Clone)]
pub struct PersonController{
    state: PersonRepositoryState,
}

impl Default for PersonController {
    fn default() -> Self {
        PersonController { state: PersonRepositoryState::default()}
    }
}

impl AsRef<PersonController> for PersonController {
   
    fn as_ref(&self) -> &Self {
        self
    }
}

impl PersonController {

    pub fn person(self) -> Router {
        async fn create(
            State(state): State<PersonRepositoryState>,
            Json(params): Json<PersonDto>,
        ) -> impl IntoResponse {
            let email: String = params.email;
            let name: String = params.name;
            let age: u8 = params.age;
            let mut write = state.write().unwrap();
            let model = Person::new(name, age, email);
            write.persons.push(model.clone());
            Json(model).into_response()
        }

        async fn get_all(State(state): State<PersonRepositoryState>) -> impl IntoResponse {
            let reader = state.read().unwrap();
            Json(reader.persons.clone()).into_response()
        }

        Router::new().route("/person", post(create).get(get_all)).with_state(self.state)
    }
}
