mod constants;
mod canvas;
mod events;
mod snake;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{rc::Rc, cell::RefCell};

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn start() {

    let mut js_canvas = canvas::JSCanvas::new();

    js_canvas.start();


    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let outer_f = f.clone();

    let window = web_sys::window().unwrap();

    if let Some(perf) = window.performance() {
        let start_time = perf.now();
        *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move || {


            js_canvas.update();
//            console_log!("the current time (in ms) is {}", perf.now());
            window.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .expect("failed requesting animation frame");
        }) as Box<dyn FnMut()>));

        let window = web_sys::window().unwrap();
        window.request_animation_frame(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("failed requesting animation frame");
    }
}

fn perf_to_system(amt: f64) -> SystemTime {
    let secs = (amt as u64) / 1_000;
    let nanos = ((amt as u32) % 1_000) * 1_000_000;
    UNIX_EPOCH + Duration::new(secs, nanos)
}