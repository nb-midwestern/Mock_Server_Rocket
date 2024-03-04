#[macro_use]
extern crate rocket;

use crate::routes::post::log_post_with_body;
use crate::routes::put::log_put_with_body;
use rocket::serde::{Deserialize, Serialize};

mod routes;
mod templates;

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
    rocket::build()
        .mount("/", routes![log_post_with_body, log_put_with_body])
        .mount("/", routes![home])
}
