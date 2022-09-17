use js_sys::Date;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::KeyboardEvent;

use super::webscoket::has_error;

fn disable_key() -> Result<(), JsValue> {
    let keydown_callback = Closure::<dyn Fn(KeyboardEvent)>::new(move |e: KeyboardEvent| {
        if e.key().to_uppercase() == "F12"
            || e.ctrl_key() && e.shift_key() && e.key().to_uppercase() == "I"
            || e.meta_key() && e.alt_key() && e.key().to_uppercase() == "I"
            || e.ctrl_key() && e.key().to_uppercase() == "U"
            || e.ctrl_key() && e.key().to_uppercase() == "S"
            || e.meta_key() && e.alt_key() && e.key().to_uppercase() == "U"
            || e.meta_key() && e.key().to_uppercase() == "S"
        {
            web_sys::console::clear();
            e.prevent_default();
        }
    });
    web_sys::window().unwrap().add_event_listener_with_callback("keydown", keydown_callback.as_ref().unchecked_ref())?;
    keydown_callback.forget();
    Ok(())
}

fn check_has_special_funs() -> Result<(), JsValue> {
    let debugger_fun = js_sys::Function::new_no_args("debugger;");
    let error_fun = js_sys::Function::new_no_args(
        "console.log(Object.defineProperties(new Error, {
  toString: {value() {(new Error).stack.includes('toString@')}},
  message: {get() {document.body.setAttribute('_DT_','true')}},
}));",
    );
    let callback = Closure::wrap(Box::new(move || {
        web_sys::console::clear();
        let now = Date::now();
        debugger_fun.call0(&"".into()).unwrap();
        error_fun.call0(&"".into()).unwrap();
        if Date::now() - now > 100f64 {
            // test only : web_sys::window().unwrap().document().unwrap().get_element_by_id("develop_tools_status").unwrap().set_inner_html("develop tools opend");
            has_error("DDT_DEBUG");
            return;
        }
        if web_sys::window().unwrap().document().unwrap().body().unwrap().get_attribute("_DT_").is_some() {
            // test only : web_sys::window().unwrap().document().unwrap().get_element_by_id("develop_tools_status").unwrap().set_inner_html("develop tools opend");
            has_error("DDT_DEBUG");
            return;
        }
    }) as Box<dyn Fn()>);
    web_sys::window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 1000).unwrap();
    callback.forget();

    Ok(())
}

pub fn init() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Enable starsys [disable dev tools] function.".into());
    disable_key()?;
    check_has_special_funs()
}
