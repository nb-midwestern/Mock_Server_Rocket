use rocket::State;

use crate::{state::save_state, AppState};

#[derive(FromForm)]
pub struct UpdateBody {
    pub value: String,
    pub field: String, // "ok" or "err"
}

#[post("/response-body", data = "<update>")]
pub fn response_body(
    update: rocket::form::Form<UpdateBody>,
    state: &State<AppState>,
) -> &'static str {
    let mut body = state.body.lock().unwrap();
    match update.field.as_str() {
        "ok" => body.ok = update.value.clone(),
        "err" => body.err = update.value.clone(),
        _ => return "invalid field",
    };
    // Save the updated state to the file
    if let Err(e) = save_state(&*body) {
        eprintln!("Failed to save state: {}", e);
    }
    "Updated"
}
