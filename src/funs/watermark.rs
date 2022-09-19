use std::{cmp, sync::Mutex};

use js_sys::Date;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, HtmlElement, MutationObserver, MutationObserverInit, MutationRecord, Node};

use crate::funs::webscoket::has_error;

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

static WATERMARK_DOM_IDS: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn generate(settings: &Setting) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let max_width = cmp::max(body.scroll_width(), body.client_width());
    let max_height = cmp::max(body.scroll_height(), body.client_height());
    let cols = (max_width as u32 - settings.start_x + settings.space_x) / (settings.width + settings.space_x);
    let rows = (max_height as u32 - settings.start_y + settings.space_y) / (settings.height + settings.space_y);
    let div = document.create_document_fragment();
    let mut ids = vec![];
    let ts = Date::now();
    for row_idx in 0..rows {
        let current_y = settings.start_y + (settings.height + settings.space_y) * row_idx;
        for col_idx in 0..cols {
            let current_x = settings.start_x + (settings.width + settings.space_x) * col_idx;
            let container = document.create_element("div")?.dyn_into::<HtmlElement>()?;
            let id = format!(
                "starsys-watermark_{}_{}_{}_{}_{}",
                current_x,
                current_y,
                current_x + settings.width,
                current_y + settings.height,
                ts
            );
            container.set_id(&id);
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
            container.style().set_property("z-index", "2147483647")?;
            container.style().set_property("pointer-events", "none")?;
            container.style().set_property("visibility", "visible")?;
            container.style().set_property("opacity", &format!("{}", settings.opacity))?;
            container.style().set_property("transform", &format!("rotate(-{}deg)", settings.angle))?;
            container.style().set_property("-moz-transform", &format!("rotate(-{}deg)", settings.angle))?;
            container.style().set_property("-ms-transform:rotate(-{}deg)", &format!("rotate(-{}deg)", settings.angle))?;
            container.style().set_property("-o-transform:rotate(-{}deg)", &format!("rotate(-{}deg)", settings.angle))?;
            div.append_child(&container)?;
            ids.push(id);
        }
    }
    let watermark_nodes = document.query_selector_all(".starsys-watermark")?;
    for n in 0..watermark_nodes.length() {
        (watermark_nodes.item(n).unwrap().dyn_into::<Element>()?).remove();
    }
    body.append_child(&div)?;
    *WATERMARK_DOM_IDS.lock().unwrap() = ids;
    Ok(())
}

fn anti_crack(records: Vec<MutationRecord>) {
    fn check(node: Option<Node>, action: &str) -> bool {
        if let Some(node) = node {
            if let Ok(element) = node.dyn_into::<HtmlElement>() {
                if WATERMARK_DOM_IDS.lock().unwrap().contains(&element.id()) {
                    if action == "ADD" {
                        return true;
                    }
                    has_error("WM_ON");
                    return false;
                }
                if let Some(css) = web_sys::window().unwrap().get_computed_style(&element).unwrap() {
                    let z_index = css.get_property_value("z-index").unwrap_or("0".to_string()).to_lowercase();
                    if z_index.is_empty() || z_index == "auto" {
                        return true;
                    }
                    if z_index.parse::<u32>().unwrap() >= 2147483647 {
                        has_error("WM_ON");
                        return false;
                    }
                }
            }
        }
        true
    }

    records.into_iter().for_each(|record| match record.type_().as_str() {
        "childList" => {
            let add_node_length = record.added_nodes().length();
            for i in 0..add_node_length {
                let _ = check(record.added_nodes().item(i), "ADD") 
                 // prevent: document.getElementById("").innerText="" or innerHTML="" 
                && check(record.target(), "INNER_ADD");
            }
            let remove_node_length = record.removed_nodes().length();
            for i in 0..remove_node_length {
                let _ = check(record.removed_nodes().item(i), "DEL") 
                 // prevent: document.getElementById("").innerText="" or innerHTML="" 
                && check(record.target(), "INNER_DEL");
            }
        }
        "characterData" => {
            // prevent: dynamically modify the watermark content in the element window of the console
            check(record.target().unwrap().parent_node(), "DATA");
        }
        "attributes" => {
            check(record.target(), "MOD");
        }
        _ => {}
    });
}

pub fn init(setting: Setting) -> Result<(), JsValue> {
    web_sys::console::log_1(&"Enable starsys [watermark] function.".into());

    if web_sys::window().unwrap().document().unwrap().ready_state() != "loading" {
        generate(&setting).unwrap();
    } else {
        let cloned_setting = setting.clone();
        let load_callback = Closure::<dyn Fn()>::new(move || {
            generate(&cloned_setting).unwrap();
        });
        web_sys::window().unwrap().add_event_listener_with_callback("DOMContentLoaded", load_callback.as_ref().unchecked_ref())?;
        load_callback.forget();
    }

    let cloned_setting = setting.clone();
    let resize_callback = Closure::<dyn Fn()>::new(move || {
        generate(&cloned_setting).unwrap();
    });
    web_sys::window().unwrap().add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())?;
    resize_callback.forget();

    let body_observer_callback = Closure::<dyn Fn(Vec<MutationRecord>, MutationObserver)>::new(move |records: Vec<MutationRecord>, _| {
        anti_crack(records);
    });
    let body_observer = MutationObserver::new(body_observer_callback.as_ref().unchecked_ref()).unwrap();
    body_observer_callback.forget();
    body_observer.observe_with_options(
        &web_sys::window().unwrap().document().unwrap().body().unwrap(),
        MutationObserverInit::new().attributes(true).subtree(true).child_list(true).character_data(true).character_data_old_value(true),
    )?;

    let cloned_setting = setting.clone();
    let watermark_observer_callback = Closure::<dyn Fn()>::new(move || {
        generate(&cloned_setting).unwrap();
    });
    let watermark_observer = MutationObserver::new(watermark_observer_callback.as_ref().unchecked_ref()).unwrap();
    watermark_observer_callback.forget();
    watermark_observer.observe_with_options(
        &web_sys::window().unwrap().document().unwrap().body().unwrap(),
        MutationObserverInit::new().attributes(true),
    )?;

    Ok(())
}
