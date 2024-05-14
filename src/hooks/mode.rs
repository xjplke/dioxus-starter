//use dioxus_local_storage::use_local_storage;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

pub fn is_dark() -> bool {
    //let storage = use_local_storage();

    let current_mode = LocalStorage::get::<String>("mode");
    if current_mode.is_err() {
        false
    } else {
        current_mode.unwrap().to_lowercase() == "dark"
    }
}

pub fn mode(dark: bool) {
    //let storage = use_local_storage(cx);
    //let state = storage.insert("mode", if dark { "dark" } else { "light" });
    let state = LocalStorage::set("mode", if dark { "dark" } else { "light" });
    if dark && state.is_ok() {
        let _ = js_sys::eval("document.documentElement.classList.add('dark');");
    } else {
        let _ = js_sys::eval("document.documentElement.classList.remove('dark');");
    }
}

pub fn init_mode_info() {
    let _ = js_sys::eval("document.body.classList.add('dark:bg-gray-600');");

    //let storage = use_local_storage(cx);
    use_hook(move || {
        let dark = LocalStorage::get("mode").unwrap_or("light".to_string()) == "dark";
        if dark {
            let _ = js_sys::eval("document.documentElement.classList.add('dark');");
        } else {
            let _ = js_sys::eval("document.documentElement.classList.remove('dark');");
        }
    });
}
