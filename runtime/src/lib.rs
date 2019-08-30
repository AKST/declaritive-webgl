pub mod math;
pub mod rendering;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast};
use web_sys::{WebGlRenderingContext};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(js_name = "start_runtime")]
pub fn start_runtime(maybe_context: JsValue) -> Result<(), JsValue> {
  return maybe_context.dyn_into::<WebGlRenderingContext>()
    .map(|context| ())
    .map_err(|value| {
      let message = format!("expected web gl context, instead got {:?}", value);
      return JsValue::from_str(message.as_ref())
    });
}

#[wasm_bindgen(js_name = "setupPanicHook")]
pub fn setup_panic_hook() {
    console_error_panic_hook::set_once();
}
