use wasm_bindgen::{JsCast, JsValue};
use js_sys::{Object, Float32Array, WebAssembly};
use web_sys::console::log_1;
use super::constants::{HasBufferKind, ViewPrecision, HasViewPrecision};

#[derive(Clone, Copy, Debug)]
pub struct Data<V: View, B: HasBufferKind> {
  pub buffer: B,
  pub view: V,
}

#[derive(Clone, Debug)]
pub struct Float32View {
  size: usize,
  data: Float32Array,
}

pub trait View: HasViewPrecision {
  fn length(&self) -> usize;
  fn object(&self) -> &Object;
  fn get_precision(&self) -> ViewPrecision;
}

impl Float32View {
  pub fn create(data_raw: &[f32]) -> Result<Self, DataViewError> {
    let data = Float32View::build_data(data_raw)?;
    Ok(Float32View { data, size: data_raw.len() })
  }

  pub fn update_data(&mut self, data_raw: &[f32]) -> Result<(), DataViewError> {
    self.data = Float32View::build_data(data_raw)?;
    self.size = data_raw.len();
    return Ok(())
  }

  pub fn log(&self) {
    let value = JsValue::from(&self.data);
    log_1(&value);
  }

  fn build_data(data_raw: &[f32]) -> Result<js_sys::Float32Array, DataViewError> {
    let memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .map_err(|_| DataViewError::FailedToCreateMemory)?
      .buffer();

    let data_location = data_raw.as_ptr() as u32 / 4;
    let data = js_sys::Float32Array::new(&memory_buffer)
      .subarray(data_location, data_location + data_raw.len() as u32);
    Ok(data)
  }
}

impl HasViewPrecision for Float32View {
  fn view_precision_constant(&self) -> u32 {
    self.get_precision().view_precision_constant()
  }
}

impl View for Float32View {
  fn length(&self) -> usize { self.size }
  fn object(&self) -> &Object { self.data.as_ref() }

  fn get_precision(&self) -> ViewPrecision {
    ViewPrecision::Float
  }
}

#[derive(Clone, Copy)]
pub enum DataViewError {
  FailedToCreateMemory,
}

impl DataViewError {
  pub fn to_string(&self) -> String {
    match self {
      DataViewError::FailedToCreateMemory => "Failed to create memory".to_string(),
    }
  }
}
