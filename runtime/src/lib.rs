pub mod render;
pub mod math;
pub mod render_loop;
pub mod ui;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast};
use web_sys::{WebGl2RenderingContext};
use render::builder::{RenderBuilder};
use render::api::{WebRenderAPI, WebRenderBuffer};
use render_loop::{RenderLoop};

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

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

type WebRenderLoop = RenderLoop<WebRenderAPI, WebRenderBuffer>;

#[derive(Debug, Copy, Clone)]
struct Dimensions {
  width: i32,
  height: i32,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Runtime {
  render_loop: WebRenderLoop,
  dimensions: Dimensions,
}

#[wasm_bindgen]
impl Runtime {
  fn new(render_loop: WebRenderLoop, dimensions: Dimensions) -> Self {
    Runtime { render_loop, dimensions }
  }

  #[wasm_bindgen]
  pub fn tick(&self) {
    self.render_loop.draw();
  }

  #[wasm_bindgen(js_name = "debugState")]
  pub fn debug_state(&self) {
    console_log!("Debug: {:#?}", self);
  }

  #[wasm_bindgen(js_name = "setDimensions")]
  pub fn set_dimensions(&mut self, width: i32, height: i32) -> Result<(), JsValue> {
    self.dimensions = Dimensions { width, height };
    self.render_loop.update_viewport(width, height).map_err(error_to_string)
  }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct RuntimeBuilder {
  dimensions: Option<Dimensions>,
  render_builder: RenderBuilder,
}

fn error_to_string<E>(error: E) -> JsValue where E: ToString {
  return JsValue::from_str(error.to_string().as_ref())
}

#[wasm_bindgen]
impl RuntimeBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Result<RuntimeBuilder, JsValue> {
    let render_builder = RenderBuilder::new();
    let dimensions = None;
    Ok(RuntimeBuilder { render_builder, dimensions })
  }

  #[wasm_bindgen(js_name = "linkWebglContext")]
  pub fn link_webgl_context(&mut self, maybe_context: JsValue) -> Result<(), JsValue> {
    return maybe_context.dyn_into::<WebGl2RenderingContext>()
      .map(|context| self.render_builder.set_context(context))
      .map_err(|value| {
        let message = format!("expected web gl context, instead got {:?}", value);
        return JsValue::from_str(message.as_ref())
      });
  }

  #[wasm_bindgen(js_name = "linkFragShader")]
  pub fn link_frag_shader(&mut self, shader_source: &str) -> Result<(), JsValue> {
    return self.render_builder.set_frag_shader(shader_source)
      .map_err(|err| JsValue::from_str(err.to_string().as_ref()))
  }

  #[wasm_bindgen(js_name = "linkVertShader")]
  pub fn link_vert_shader(&mut self, shader_source: &str) -> Result<(), JsValue> {
    return self.render_builder.set_vert_shader(shader_source)
      .map_err(|err| JsValue::from_str(err.to_string().as_ref()))
  }

  #[wasm_bindgen(js_name = "createRuntime")]
  pub fn create_runtime(&mut self) -> Result<Runtime, JsValue> {
    let dimensions = self.dimensions.ok_or("need dimensions before building runtime")?;
    let render_loop = self.render_builder.build_render_api()
      .map_err(error_to_string)
      .and_then(|render_api|
          RenderLoop::create(
            render_api,
            dimensions.width,
            dimensions.height,
          ).map_err(error_to_string))?;

    Ok(Runtime::new(render_loop, dimensions))
  }

  #[wasm_bindgen(js_name = "setDimensions")]
  pub fn set_dimensions(&mut self, width: i32, height: i32) {
    self.dimensions = Some(Dimensions { width, height });
  }

  #[wasm_bindgen(js_name = "debugState")]
  pub fn debug_state(&self) {
    console_log!("Debug: {:#?}", self);
  }
}


#[wasm_bindgen(js_name = "setupPanicHook")]
pub fn setup_panic_hook() {
    console_error_panic_hook::set_once();
}
