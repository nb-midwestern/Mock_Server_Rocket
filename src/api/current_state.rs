use crate::{AppState, ResponseBody};
use rocket::serde::json::Json;
use rocket::State;

#[get("/current-values")]
pub fn current_values(state: &State<AppState>) -> Json<ResponseBody> {
    let body = state.body.lock().unwrap();
    Json(ResponseBody {
        ok: body.ok.clone(),
        err: body.err.clone(),
    })
}
