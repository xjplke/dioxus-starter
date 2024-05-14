#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
//use dioxus_toast::{ToastFrame, ToastManager};

mod components;
mod hooks;
mod pages;

//use fermi::{use_atom_ref, use_init_atom_root, AtomRef};
//use hooks::mode::init_mode_info;
use pages::starter::{About, HelloDioxus, SayHi};
use pages::_404::NotFound;

//static TOAST_MANAGER: AtomRef<ToastManager> = AtomRef(|_| ToastManager::default());
//static NAME: GlobalSignal<String> = Signal::global(|| "world".to_string());
//static TOAST_MANAGER: GlobalSignal<ToastManager> = Signal::global(|| ToastManager::default());

#[derive(Routable, Clone)]
enum Route {
    // Main Page
    #[route("/")]
    HelloDioxus {},

    // Say Hi Page
    #[route("/hi/:name")]
    SayHi { name: String },

    // About Page
    #[route("/about")]
    About {},

    // 404 Not Found Page
    #[route("/:route")]
    NotFound { route: String },
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Dioxus Starter: https://github.com/mrxiaozhuox/dioxus-starter");
    launch_web(App)
}

fn App() -> Element {
    // init fermi atom root
    //use_init_atom_root(&cx);
    //use_init_atom_root();

    // init mode information
    //init_mode_info(&cx);

    rsx! {
        // dioxus toast manager init
        //ToastFrame { manager: TOAST_MANAGER }
        // dioxus router init
        Router::<Route> { }
    }
}
