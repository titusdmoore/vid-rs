use eye::colorconvert::Device;
use eye_hal::format::PixelFormat;
use eye_hal::traits::{Context as _, Device as _, Stream as _};
use eye_hal::PlatformContext;
use leptos::html::Canvas;
use leptos::*;
use std::sync::mpsc;
use std::thread;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn render_image(canvas_ref: &NodeRef<Canvas>, buf: Vec<u8>) {
    let canvas = canvas_ref.get().unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    for (i, pixel) in buf.chunks_exact(3).enumerate() {
        let x = i % 1280;
        let y = i / 1280;

        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        ctx.set_fill_style(&format!("rgba({}, {}, {}, 255)", r, g, b).into());
        ctx.fill_rect(x as f64, y as f64, 1.0, 1.0);
    }
}

#[component]
pub fn App() -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();
    let (canvas_content, set_canvas_content) = create_signal(Vec::<u8>::new());

    // thread::spawn(move || loop {
    //     let buf = rx.recv().unwrap();
    //     set_canvas_content.update(|x| *x = buf);
    // });
    create_effect(move |_| {
        render_image(&canvas_ref, canvas_content.get());
    });

    println!("Started stream");
    let devices = vec![1, 2, 3, 4];

    view! {
        <main class="container">
            <h1>"Hello, world!"</h1>
            <p>{devices.len()}</p>
        </main>
    }
}
