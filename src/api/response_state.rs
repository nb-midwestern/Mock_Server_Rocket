use rocket::form::Form;
use std::sync::Mutex;

use crate::ResponseState;

#[derive(FromForm)]
pub struct ResponseStateToggle {
    pub state: bool,
}

#[post("/response-type/toggle", data = "<toggle>")]
pub fn response_state_toggle(
    toggle: Form<ResponseStateToggle>,
    state: &rocket::State<ResponseState>,
) -> String {
    let mut value = state.value.lock().unwrap();
    *value = toggle.state;
    let response = if *value {
        r#"
          <div id="toggle-status" class="notification is-info">
              Returning Success 200
          </div>

          "#
    } else {
        r#"
          <div id="toggle-status" class="notification is-info">
              Returning 400 Bad Request
          </div>

          "#
    };
    response.to_string()
}
