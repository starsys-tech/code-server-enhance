use std::sync::Mutex;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{ClipboardEvent, Event, HtmlInputElement, HtmlTextAreaElement};

use crate::utils::crypt::{decrypt, encrypt};

fn cut_or_copy_procssor(e: ClipboardEvent) {
    e.prevent_default();
    e.stop_immediate_propagation();
    let selection = web_sys::window().unwrap().get_selection().expect("Unable to get selection object").unwrap();
    set_seed();
    let encrypt_text = encrypt(&SEED.lock().unwrap(), &selection.to_string().as_string().unwrap());
    *DATA.lock().unwrap() = encrypt_text;
    e.clipboard_data().unwrap().set_data("text/plain", "__STARSYS__").unwrap();
    if e.type_().to_lowercase() == "cut" {
        selection.delete_from_document().unwrap();
    }
}

pub fn paste_data_process(text: &str) -> String {
    if text == "__STARSYS__" {
        let t = DATA.lock().unwrap();
        if !t.is_empty() {
            return decrypt(&SEED.lock().unwrap(), t.as_str());
        }
    }
    text.to_string()
}

fn paste_procssor(e: ClipboardEvent) {
    e.prevent_default();
    e.stop_immediate_propagation();
    let text = e.clipboard_data().unwrap().get_data("text/plain").unwrap();
    let text = paste_data_process(&text);

    let target = e.target().unwrap();
    if let Some(ele) = target.dyn_ref::<HtmlInputElement>() {
        ele.set_value(&text);
    } else if let Some(ele) = target.dyn_ref::<HtmlTextAreaElement>() {
        ele.set_value(&text);
    } else {
        web_sys::console::trace_1(&format!("Paste to {}", target.to_string().as_string().unwrap().as_str()).into());
    }
}

pub fn init() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Enable starsys [clipboard] function.".into());
    let window = web_sys::window().unwrap();
    let cut_or_copy_fn = Closure::<dyn Fn(Event)>::new(|e: Event| cut_or_copy_procssor(e.dyn_into::<ClipboardEvent>().unwrap()));
    let paste_fn = Closure::<dyn Fn(Event)>::new(|e: Event| paste_procssor(e.dyn_into::<ClipboardEvent>().unwrap()));
    window.add_event_listener_with_callback("cut", cut_or_copy_fn.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("copy", cut_or_copy_fn.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("paste", paste_fn.as_ref().unchecked_ref())?;
    cut_or_copy_fn.forget();
    paste_fn.forget();
    Ok(())
}

static SEED: Mutex<String> = Mutex::new(String::new());
static DATA: Mutex<String> = Mutex::new(String::new());

fn set_seed() {
    let mut buf = [0u8; 4];
    getrandom::getrandom(&mut buf).unwrap();
    let seed = buf.to_vec().into_iter().map(|i| i.to_string()).collect::<String>();
    *SEED.lock().unwrap() = seed;
}

#[cfg(test)]
mod tests {
    use crate::funs::clipboard::{set_seed, SEED};

    #[test]
    fn test_rand() {
        set_seed();
        println!("2={}", SEED.lock().unwrap());
    }
}
