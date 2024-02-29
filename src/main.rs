#[macro_use]
extern crate rocket;

use log::info;
use rocket::data::{Data, ToByteUnit};
use rocket::http::ContentType;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::io::AsyncReadExt;
use serde_json::Value;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct LogBody {
    data: String,
}

#[post("/<path..>", data = "<data>")]
async fn log_body(path: PathBuf, content_type: &ContentType, data: Data<'_>) -> () {
    let mut body = String::new();
    if let Err(_) = data.open(1.mebibytes()).read_to_string(&mut body).await {
        println!("ERROR");

        return;
    }

    match content_type.is_json() {
        true => {
            let json: Value = serde_json::from_str(body.as_str()).unwrap();
            println!("Logged Json Body for path {}: \n {}", path.display(), body);
            println!("\n\n{}\n\n", serde_json::to_string_pretty(&json).unwrap(),)
        }
        _ => {
            println!(
                "Logged {} Body for path {}: {}",
                content_type,
                path.display(),
                body
            );
        }
    }
}

#[get("/")]
async fn home() -> () {
    info!("home route hit");
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![log_body, home])
}
