use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

use crate::ResponseBody;

const STATE_FILE: &str = "state.json";

pub fn load_state() -> io::Result<ResponseBody> {
    let path = Path::new(STATE_FILE);
    if path.exists() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let state: ResponseBody = serde_json::from_str(&contents)?;
        Ok(state)
    } else {
        Ok(ResponseBody {
            ok: String::new(),
            err: String::new(),
        })
    }
}

pub fn save_state(state: &ResponseBody) -> io::Result<()> {
    let contents = serde_json::to_string(state)?;
    fs::write(STATE_FILE, contents)?;
    Ok(())
}
