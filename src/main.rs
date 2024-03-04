#[macro_use]
extern crate rocket;

use std::sync::Mutex;

use crate::routes::post::log_post_with_body;
use crate::routes::put::log_put_with_body;
use api::{
    current_state::{self, current_values},
    response_body::response_body,
    response_state::response_state_toggle,
};
use rocket::serde::{Deserialize, Serialize};

mod api;
mod routes;
mod state;
mod templates;

pub struct ResponseState {
    pub value: Mutex<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseBody {
    pub ok: String,
    pub err: String,
}

pub struct AppState {
    pub body: Mutex<ResponseBody>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct LogBody {
    data: String,
}

#[get("/")]
async fn home() -> templates::Index {
    templates::Index {
        title: "home".to_string(),
    }
}

#[launch]
fn rocket() -> _ {
    let state = state::load_state().expect("Failed to load state");
    rocket::build()
        .mount(
            "/api/",
            routes![current_values, response_state_toggle, response_body],
        )
        .mount("/", routes![log_post_with_body, log_put_with_body])
        .mount("/", routes![home])
        .manage(ResponseState {
            value: Mutex::new(false),
        })
        .manage(AppState {
            body: Mutex::new(state),
        })
}
