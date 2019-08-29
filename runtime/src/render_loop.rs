use super::render::api::{
  AttributeKey,
  RenderAPI,
  RenderApiError,
  UniformKey,
};
use super::render::data::{Float32View, DataViewError, View};
use super::render::constants::{
  BufferKind,
  DrawArrayKind,
  DrawKind,
  ClearMask,
  HasBufferKind,
};

#[derive(Clone, Copy, Debug)]
enum VertexAttributes {
  Position,
}

#[derive(Clone, Copy, Debug)]
enum VertexUniforms {
  Resoultion,
}

impl AttributeKey for VertexAttributes {
  fn name(&self) -> &str {
    match self {
      VertexAttributes::Position => "position"
    }
  }
}

impl UniformKey for VertexUniforms {
  fn name(&self) -> &str {
    match self {
      VertexUniforms::Resoultion => "resolution"
    }
  }
}

#[derive(Debug)]
pub struct RenderLoop<R, B> {
  view: Float32View,
  buffer: B,
  context: R,
}

fn grid_points(width: i32, height: i32, row_len: u32, col_len: u32) -> Vec<f32> {
  let width_f = width as f32;
  let height_f = height as f32;

  let border = (width_f / (row_len as f32)) * 0.075;
  let grid_outer = (width_f - border) / (row_len as f32);
  let grid_inner = grid_outer - border;

  // the distance from the top to center the grid in the screen
  let height_offset = (height_f / 2.0) - ((col_len as f32 * grid_outer) / 2.0);

  let write_point = |write_to: &mut [f32], x_offset: f32, y_offset: f32| {
    write_to[0] = x_offset;
    write_to[1] = y_offset;
    write_to[2] = 0.0;
  };

  let write_triangle = |triangle: &mut [f32], x: f32, y: f32, length: f32| {
    write_point(&mut triangle[0..3], x, y);
    write_point(&mut triangle[3..6], x + length, y);
    write_point(&mut triangle[6..9], x, y + length);
  };

  let make_square_points = |row_start: f32, col_start: f32| -> [f32; 18] {
    let mut square: [f32; 18] = [0.0; 18];
    let length = grid_inner;
    let row_offset = (row_start * grid_outer) + border;
    let col_offset = (col_start * grid_outer) + border + height_offset;
    write_triangle(&mut square[0..9], row_offset, col_offset, length);
    write_triangle(&mut square[9..18], row_offset + length, col_offset + length, -length);
    square
  };

  let mut grid = vec![];
  for r_index in 0..row_len {
    for c_index in 0..col_len {
      let square_points = make_square_points(r_index as f32, c_index as f32);
      grid.extend_from_slice(&square_points);
    }
  }

  grid
}

fn get_view_data(width: i32, height: i32) -> Vec<f32> {
  grid_points(width, height, 12, 3)
}

impl<R, B> RenderLoop<R, B> where R: RenderAPI<Buffer=B>, B: HasBufferKind {
  pub fn create(context: R, width: i32, height: i32) -> Result<Self, RenderLoopError> {
    let buffer = context.create_buffer(BufferKind::ArrayBuffer)?;
    let data = get_view_data(width, height);
    let view = Float32View::create(&data)?;
    context.bind_buffer(&buffer, &view, DrawKind::StaticDraw);

    let position = VertexAttributes::Position;
    let precision = view.get_precision();
    context.vertex_attrib_pointer_with_i32(position, 3, precision, false, 0, 0)?;
    context.enable_vertex_attrib_array(position)?;

    let resolution = VertexUniforms::Resoultion;
    context.uniform2f(resolution, width as f32, height as f32)?;

    Ok(RenderLoop { buffer, view, context })
  }

  pub fn draw(&self) {
    self.context.clear_color(0.0, 0.0, 0.0, 1.0);
    self.context.clear(ClearMask::ColorBufferBit);

    let count = (self.view.length() / 3) as i32;
    self.context.draw_arrays(DrawArrayKind::Triangles, 0, count);
  }

  pub fn update_viewport(&mut self, width: i32, height: i32) -> Result<(), RenderLoopError> {
    let data = get_view_data(width, height);

    self.context.set_viewport(0, 0, width, height);
    self.view.update_data(&data)?;
    self.context.bind_buffer(&self.buffer, &self.view, DrawKind::StaticDraw);

    let resolution = VertexUniforms::Resoultion;
    self.context.uniform2f(resolution, width as f32, height as f32)?;

    return Ok(());
  }
}

pub enum RenderLoopError {
  RenderApiError(RenderApiError),
  DataViewError(DataViewError),
}

impl From<RenderApiError> for RenderLoopError {
  fn from(error: RenderApiError) -> Self {
    RenderLoopError::RenderApiError(error)
  }
}

impl From<DataViewError> for RenderLoopError {
  fn from(error: DataViewError) -> Self {
    RenderLoopError::DataViewError(error)
  }
}

impl ToString for RenderLoopError {
  fn to_string(&self) -> String {
    match self {
      RenderLoopError::RenderApiError(e) => format!("render_loop RenderApiError: {}", e.to_string()),
      RenderLoopError::DataViewError(e) => format!("render_loop DataViewError: {}", e.to_string()),
    }
  }
}
