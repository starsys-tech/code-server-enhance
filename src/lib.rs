mod funs;
mod utils;
use std::sync::Mutex;

use funs::{
    clipboard, task,
    watermark::{self, Setting},
    webscoket, disable_dev_tools,
};
use wasm_bindgen::prelude::*;

struct Config {
    pub ak: String,
    pub cc: String,
}

pub(crate) static CONFIG: Mutex<Config> = Mutex::new(Config {
    ak: String::new(),
    cc: String::new(),
});

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Enable starsys functions.".into());
    let window = web_sys::window().unwrap();
    console_error_panic_hook::set_once();
    clipboard::init()?;
    webscoket::init(&format!("wss://{}/starsys", window.location().host()?), |ak: &str, cc: &str| {
        watermark::init(Setting {
            txt1: ak.to_string(),
            txt2: cc.to_string(),
            ..Default::default()
        })
        .unwrap();
        ()
    })?;
    task::init()?;
    disable_dev_tools::init()
}

#[wasm_bindgen]
pub fn paste_data_process(text: &str) -> String {
    return clipboard::paste_data_process(text);
}
