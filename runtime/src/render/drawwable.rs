use web_sys::WebGl2RenderingContext;

pub trait Drawwable {
  // TODO(Angus): replace with api
  fn draw(&self, context: &WebGl2RenderingContext) -> Result<(), DrawError>;
}

pub enum DrawError {
}
