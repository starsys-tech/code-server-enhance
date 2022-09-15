use std::{str::FromStr, sync::Mutex};

use js_sys::Date;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{MessageEvent, WebSocket};

use crate::{
    utils::crypt::{decrypt, encrypt},
    Config, CONFIG,
};

static ERROR: Mutex<String> = Mutex::new(String::new());

fn process_init(check_period_ms: i32, ws: WebSocket) {
    let window = web_sys::window().unwrap();
    let callback = Closure::wrap(Box::new(move || {
        let conf = CONFIG.lock().unwrap();
        let now = Date::now();
        let check_code = format!("{}{}", now, encrypt(&now.to_string(), &format!("{}{}", conf.ak, conf.cc)));
        ws.send_with_str(&format!("checker,{}", check_code)).unwrap();
    }) as Box<dyn Fn()>);
    window.set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), check_period_ms).unwrap();
    callback.forget();
}

fn process_checker(check_code: &str) -> bool {
    let timestamp = &check_code[0..13];
    let encrypt_text = &check_code[13..];
    let conf = CONFIG.lock().unwrap();
    decrypt(timestamp, encrypt_text) == format!("{}{}", conf.ak, conf.cc)
}

pub fn has_error(kind: &str) {
    *ERROR.lock().unwrap() = kind.to_string()
}

pub fn init<F>(url: &str, init_fn: F) -> Result<(), JsValue>
where
    F: Fn(&str, &str) -> () + 'static,
{
    web_sys::console::log_1(&"Enable starsys [websocket] function.".into());
    let ws = WebSocket::new(url)?;

    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::<dyn Fn(MessageEvent)>::new(move |e: MessageEvent| {
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            let txt = txt.as_string().unwrap();
            let items = txt.split(",").collect::<Vec<&str>>();
            match items[0] {
                "init" => {
                    let ak = items[1];
                    let cc = items[2];
                    let check_period_ms = FromStr::from_str(items[3]).unwrap();
                    let mut conf = CONFIG.lock().unwrap();
                    *conf = Config {
                        ak: ak.to_string(),
                        cc: cc.to_string(),
                    };
                    init_fn(ak, cc);
                    process_init(check_period_ms, cloned_ws.clone());
                }
                "checker" => {
                    let error_kind = ERROR.lock().unwrap();
                    if !error_kind.is_empty() {
                        cloned_ws.send_with_str(&format!("close,{}", error_kind.to_string())).unwrap();
                        return;
                    }
                    let check_code = items[1];
                    if !process_checker(check_code) {
                        cloned_ws.send_with_str("close,CC").unwrap();
                    }
                }
                _ => {}
            }
        } else {
            web_sys::console::log_1(&format!("message event, received Unknown: {:?}", e.data()).as_str().into());
        }
    });
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::<dyn Fn()>::new(move || cloned_ws.send_with_str("init").unwrap());
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}
