mod line;
mod scatter;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

// Read from csv
pub async fn read_csv_rust(data: web_sys::File) -> Vec<f64> {
    let jsdata = wasm_bindgen_futures::JsFuture::from(data.text())
        .await
        .unwrap_throw();

    let jsdatas = jsdata.as_string().unwrap_throw();

    let mut reader = csv::Reader::from_reader(jsdatas.as_bytes());

    let mut data = Vec::new();
    for record in reader.records() {
        for field in record.unwrap().iter() {
            let value = f64::from_str(field);
            data.push(value.unwrap());
        }
    }

    return data;
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function at least once during initialization, and then
// we will get better error messages if our code ever panics.
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Run when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}
