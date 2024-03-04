use chrono::Local;
use rocket::data::{Data, ToByteUnit};
use rocket::http::ContentType;
use rocket::response::status::BadRequest;
use rocket::tokio::io::AsyncReadExt;
use serde_json::Value;
use std::path::PathBuf;

#[put("/<path..>", data = "<data>")]
pub async fn log_put_with_body(
    path: PathBuf,
    content_type: &ContentType,
    data: Data<'_>,
) -> Result<String, BadRequest<String>> {
    let now = Local::now();
    let mut body = String::new();
    println!("Request made at: {}", now);
    if let Err(_) = data.open(1.mebibytes()).read_to_string(&mut body).await {
        println!("ERROR");
        return Ok("Error".to_string());
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
    // Ok("{}".to_string())
    Err(BadRequest("Bad request".to_string()))
}
