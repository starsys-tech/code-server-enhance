use std::cmp;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Event, HtmlElement, HtmlInputElement, HtmlTextAreaElement, KeyboardEvent, Node};

fn generate_task_container() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    let task_container = document.create_element("div")?.dyn_into::<HtmlElement>()?;
    task_container.set_id("starsys-task-container");
    task_container.style().set_property("position", "absolute").unwrap();
    task_container.style().set_property("top", "0").unwrap();
    task_container.style().set_property("left", "0").unwrap();
    task_container.style().set_property("right", "0").unwrap();
    task_container.style().set_property("bottom", "0").unwrap();
    task_container.style().set_property("z-index", "9999").unwrap();
    task_container.style().set_property("margin", "auto").unwrap();
    task_container.style().set_property("display", "none").unwrap();
    task_container.style().set_property("font-size", "25px").unwrap();
    document.body().unwrap().append_child(&task_container)?;

    let search_container = document.create_element("input")?.dyn_into::<HtmlInputElement>()?;
    search_container.set_id("starsys-task-search");
    search_container.set_placeholder("Search Tasks");
    search_container.style().set_property("text-align", "center").unwrap();
    search_container.style().set_property("font-size", "25px").unwrap();
    search_container.style().set_property("padding", "4px").unwrap();
    search_container.style().set_property("width", "100%").unwrap();
    search_container.style().set_property("border-width", "7px").unwrap();
    search_container.style().set_property("border-style", "solid").unwrap();
    search_container.style().set_property("border-color", "#DFDFDF").unwrap();
    search_container.style().set_property("border-radius", "10px").unwrap();
    search_container.style().set_property("box-sizing", "border-box").unwrap();
    task_container.append_child(&search_container)?;

    let list_container = document.create_element("div")?.dyn_into::<HtmlElement>()?;
    list_container.set_id("starsys-task-list");
    list_container.style().set_property("overflow-x", "hidden").unwrap();
    list_container.style().set_property("overflow-y", "auto").unwrap();
    list_container.style().set_property("margin-top", "6px").unwrap();
    list_container.style().set_property("border-width", "2px").unwrap();
    list_container.style().set_property("border-style", "solid").unwrap();
    list_container.style().set_property("border-color", "#DFDFDF").unwrap();
    list_container.style().set_property("border-radius", "10px").unwrap();
    task_container.append_child(&list_container)?;

    Ok(())
}

fn process_task() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let task_container = document.get_element_by_id("starsys-task-container").unwrap().dyn_into::<HtmlElement>().unwrap();
    let search_container = document.get_element_by_id("starsys-task-search").unwrap().dyn_into::<HtmlInputElement>().unwrap();
    let list_container = document.get_element_by_id("starsys-task-list").unwrap().dyn_into::<HtmlElement>().unwrap();

    let cloned_document = document.clone();
    let cloned_task_container = task_container.clone();
    let cloned_search_container = search_container.clone();
    let cloned_list_container = list_container.clone();
    let click_callback = Closure::<dyn Fn(Event)>::new(move |e: Event| {
        if e.target().is_none() {
            return;
        }
        let ele_target = e.target().unwrap();
        if ele_target.dyn_ref::<HtmlElement>().is_none() {
            return;
        }
        let ele_target = ele_target.dyn_into::<HtmlElement>().unwrap();

        if ele_target.class_name() == "starsys-task-item" {
            let scm_editor_text_area = cloned_document.query_selector(".scm-editor textarea").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
            scm_editor_text_area.set_value(&ele_target.inner_text());
            scm_editor_text_area.style().set_property("display", "none").unwrap();
            scm_editor_text_area.dispatch_event(&Event::new("input").unwrap()).unwrap();
            cloned_task_container.style().set_property("display", "none").unwrap();
            return;
        }

        let mut target_from_editor: bool = false;
        let mut target_from_task_container: bool = false;

        let mut parent_node: Node = ele_target.into();
        loop {
            if let Some(ele) = parent_node.dyn_ref::<HtmlElement>() {
                if ele.class_name() == "scm-editor" {
                    target_from_editor = true;
                    break;
                }
                if ele.id() == "starsys-task-container" {
                    target_from_task_container = true;
                    break;
                }
            }
            if let Some(node) = parent_node.parent_node() {
                parent_node = node;
            } else {
                break;
            }
        }

        if target_from_editor {
            let scm_editor_text_area = cloned_document.query_selector(".scm-editor textarea").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
            scm_editor_text_area.set_read_only(true);
            let max_height = cmp::max(web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap() as i32 / 2, 200);
            let max_height_with_search = cloned_search_container.scroll_height() + max_height;
            let width = cmp::max(web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap() as i32 / 2, 250);
            cloned_task_container.style().set_property("width", &format!("{}px", width)).unwrap();
            cloned_task_container.style().set_property("max_height", &format!("{}px", max_height_with_search)).unwrap();
            cloned_task_container.style().set_property("display", "block").unwrap();

            cloned_list_container.style().set_property("max_height", &format!("{}px", max_height)).unwrap();
            cloned_list_container.style().set_property("display", "none").unwrap();
            cloned_list_container.style().set_property("background-color", "#fff").unwrap();
            cloned_list_container.set_inner_text("");

            cloned_search_container.set_value("");
            return;
        }

        if target_from_task_container {
            return;
        }

        cloned_task_container.style().set_property("display", "none").unwrap();
    });
    body.add_event_listener_with_callback("click", click_callback.as_ref().unchecked_ref())?;
    click_callback.forget();

    let task_container_clone = task_container.clone();
    let keydown_callback = Closure::<dyn Fn(KeyboardEvent)>::new(move |e: KeyboardEvent| {
        if e.key() == "Escape" {
            task_container_clone.style().set_property("display", "none").unwrap();
        }
    });
    body.add_event_listener_with_callback("keydown", keydown_callback.as_ref().unchecked_ref())?;
    keydown_callback.forget();

    let cloned_document = document.clone();
    let cloned_list_container = list_container.clone();
    let input_callback = Closure::<dyn Fn()>::new(move || {
        let item = cloned_document.create_element("p").unwrap().dyn_into::<HtmlElement>().unwrap();
        item.set_class_name("starsys-task-item");
        item.set_inner_text("测试任务");
        item.style().set_property("margin", "10px 10px").unwrap();
        item.style().set_property("padding", "2px").unwrap();
        cloned_list_container.append_child(&item).unwrap();
        cloned_list_container.style().set_property("display", "block").unwrap();
    });
    search_container.add_event_listener_with_callback("input", input_callback.as_ref().unchecked_ref())?;
    input_callback.forget();
    Ok(())
}

pub fn init() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Enable starsys [task] function.".into());

    if web_sys::window().unwrap().document().unwrap().ready_state() != "loading" {
        generate_task_container().unwrap();
        process_task().unwrap();
    } else {
        let load_callback = Closure::<dyn Fn()>::new(|| {
            generate_task_container().unwrap();
            process_task().unwrap();
        });
        web_sys::window().unwrap().add_event_listener_with_callback("DOMContentLoaded", load_callback.as_ref().unchecked_ref())?;
        load_callback.forget();
    }

    Ok(())
}
