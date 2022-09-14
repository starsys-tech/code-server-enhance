use std::cmp;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, HtmlElement, MutationObserver, MutationObserverInit};

#[derive(Clone)]
pub struct Setting {
    pub txt1: String,
    pub txt2: String,
    pub start_x: u32,
    pub start_y: u32,
    pub space_x: u32,
    pub space_y: u32,
    pub width: u32,
    pub height: u32,
    pub color: String,
    pub opacity: f32,
    pub fontsize: String,
    pub angle: u32,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            txt1: "text 1".to_string(),
            txt2: "text 2".to_string(),
            start_x: 20,
            start_y: 20,
            space_x: 50,
            space_y: 50,
            width: 200,
            height: 80,
            color: "#aaa".to_string(),
            opacity: 0.2,
            fontsize: "15px".to_string(),
            angle: 15,
        }
    }
}

pub fn generate(settings: &Setting) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let max_width = cmp::max(body.scroll_width(), body.client_width());
    let max_height = cmp::max(body.scroll_height(), body.client_height());
    web_sys::console::log_1(&format!("--------------x:{},{}", max_width, max_height).into());

    let cols = (max_width as u32 - settings.start_x + settings.space_x) / (settings.width + settings.space_x);
    //let space_x = (max_width as u32 - settings.start_x + settings.width * cols) / (cols - 1);
    let rows = (max_height as u32 - settings.start_y + settings.space_y) / (settings.height + settings.space_y);
    //let space_y = (max_height as u32 - settings.start_y - settings.height * rows) / (rows - 1);
    let div = document.create_document_fragment();
    for row_idx in 0..rows {
        let current_y = settings.start_y + (settings.height + settings.space_y) * row_idx;
        for col_idx in 0..cols {
            let current_x = settings.start_x + (settings.width + settings.space_x) * col_idx;
            let container = document.create_element("div")?.dyn_into::<HtmlElement>()?;
            container.set_class_name("starsys-watermark");
            container.set_inner_html(&format!("{}<br/>{}", settings.txt1, settings.txt2));
            container.style().set_property("font-size", &settings.fontsize)?;
            container.style().set_property("color", &settings.color)?;
            container.style().set_property("text-align", "center")?;
            container.style().set_property("display", "block")?;
            container.style().set_property("left", &format!("{}px", current_x))?;
            container.style().set_property("top", &format!("{}px", current_y))?;
            container.style().set_property("width", &format!("{}", settings.width))?;
            container.style().set_property("height", &format!("{}", settings.height))?;
            container.style().set_property("position", "absolute")?;
            container.style().set_property("overflow", "hidden")?;
            container.style().set_property("z-index", "9999")?;
            container.style().set_property("pointer-events", "none")?;
            container.style().set_property("opacity", &format!("{}", settings.opacity))?;
            container.style().set_property("transform", &format!("rotate(-{}deg)", settings.angle))?;
            container.style().set_property("-moz-transform", &format!("rotate(-{}deg)", settings.angle))?;
            container.style().set_property("-ms-transform:rotate(-{}deg)", &format!("rotate(-{}deg)", settings.angle))?;
            container.style().set_property("-o-transform:rotate(-{}deg)", &format!("rotate(-{}deg)", settings.angle))?;
            div.append_child(&container)?;
        }
    }
    let watermark_nodes = document.query_selector_all(".starsys-watermark")?;
    for n in 0..watermark_nodes.length() {
        (watermark_nodes.item(n).unwrap().dyn_into::<Element>()?).remove();
    }
    body.append_child(&div)?;
    Ok(())
}

pub fn init(setting: Setting) -> Result<(), JsValue> {
    web_sys::console::log_1(&"Enable starsys [watermark] function.".into());

    let cloned_setting = setting.clone();
    let resize_callback = Closure::<dyn Fn()>::new(move || {
        generate(&cloned_setting).unwrap();
    });
    web_sys::window().unwrap().add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())?;
    resize_callback.forget();

    let observer_callback = Closure::<dyn Fn()>::new(move || {
        generate(&setting).unwrap();
    });
    let observer = MutationObserver::new(observer_callback.as_ref().unchecked_ref()).unwrap();
    observer_callback.forget();
    observer.observe_with_options(
        &web_sys::window().unwrap().document().unwrap().body().unwrap(),
        MutationObserverInit::new().attributes(true),
    )?;

    Ok(())
}
