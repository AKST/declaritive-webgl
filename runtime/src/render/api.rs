use std::convert::TryFrom;
use web_sys::{
  WebGlBuffer,
  WebGlProgram,
  WebGl2RenderingContext,
  WebGlUniformLocation,
};
use super::constants::{
  BufferKind,
  BlendFuncFactor,
  ClearMask,
  DrawArrayKind,
  DrawKind,
  ViewPrecision,
  HasBufferKind,
  HasBlendFuncFactor,
  HasViewPrecision,
  HasClearMaskKind,
  HasDrawArrayKind,
  HasDrawKind,
};
use super::data::{View};

type AttributeIndex = u32;

pub trait AttributeKey {
  fn name(&self) -> &str;
}

pub trait UniformKey {
  fn name(&self) -> &str;
}

pub trait IntoAttributeIndex {
  fn with_context<C>(self, context: &C) -> Result<AttributeIndex, RenderApiError> where C: RenderAPI;
}

pub trait IntoUniformIndex {
  fn with_context<C>(self, context: &C) -> Result<C::UniformIndex, RenderApiError> where C: RenderAPI;
}

/**
 * A wrapper around `WebGlRenderingContext` to reduce the
 * number of repetitive constants, but also adds a bit more
 * type safety with enums in the place of u32 constants, and
 * requires attributes & uniforms to be accessed via your
 * own defined enum to avoid passing around strings.
 *
 * See here for more on `WebGlRenderingContext`:
 *
 * https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html
 */
pub trait RenderAPI {
  type Buffer: HasBufferKind;
  type UniformIndex;

  /**
   * Wrapper around `WebGlRenderingContext::bind_buffer`.
   *
   * https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.bind_buffer
   */
  fn bind_buffer<V>(
      &self,
      buffer: &Self::Buffer,
      view: &V,
      draw_kind: DrawKind,
  ) where V: View;

  /**
   * Wrapper around `WebGlRenderingContext::blend_color`.
   *
   * https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendColor
   */
  fn blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32);

  /**
   * Wrapper around `WebGlRenderingContext::blend_func`.
   *
   * https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendFunc
   */
  fn blend_func(&self, src: BlendFuncFactor, dst: BlendFuncFactor);

  /**
   * Wrapper around `WebGlRenderingContext::clear_color`.
   *
   * https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.clear_color
   */
  fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32);

  /**
   * Wrapper around `WebGlRenderingContext::clear`.
   *
   * https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.clear
   */
  fn clear(&self, mask: ClearMask);

  /**
   * Wrapper around `WebGlRenderingContext::create_buffer`.
   *
   * https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.create_buffer
   */
  fn create_buffer(
      &self,
      kind: BufferKind,
  ) -> Result<Self::Buffer, RenderApiError>;

  /**
   * Wrapper around `WebGlRenderingContext::draw_arrays`.
   *
   * https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.draw_arrays
   */
  fn draw_arrays(&self, mode: DrawArrayKind, first: i32, count: i32);

  /**
   * Wrapper around `WebGlRenderingContext::enable_vertex_attrib_array`.
   *
   * https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.enable_vertex_attrib_array
   */
  fn enable_vertex_attrib_array<A>(&self, key: A) -> Result<(), RenderApiError>
      where A: IntoAttributeIndex;

  /**
   * Type safe way of retrieving attribute indexs from the shader.
   */
  fn get_attribute<AK>(&self, key: AK) -> Result<AttributeIndex, RenderApiError>
      where AK: AttributeKey;

  /**
   * Type safe way of retrieving uniform indexs from the shader.
   */
  fn get_uniform<UK>(&self, key: UK) -> Result<Self::UniformIndex, RenderApiError>
      where UK: UniformKey;

  /**
   * Updates the viewport for the shader.
   */
  fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32);

  /**
   * Type safe wrapper for `uniformf2`
   *
   * https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.uniform2f
   */
  fn uniform2f<U>(&self, key: U, x: f32, y: f32) -> Result<(), RenderApiError> where U: IntoUniformIndex;

  /**
   * Wrapper around `WebGlRenderingContext::vertex_attrib_pointer_with_i32`.
   *
   * https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.vertex_attrib_pointer_with_i32
   */
  fn vertex_attrib_pointer_with_i32<A>(
      &self,
      key: A,
      size: i32,
      precision: ViewPrecision,
      normalized: bool,
      stride: i32,
      offset: i32
  ) -> Result<(), RenderApiError> where A: IntoAttributeIndex;
}

#[derive(Debug)]
pub struct WebRenderAPI {
  gl: WebGl2RenderingContext,
  program: WebGlProgram,
}

impl WebRenderAPI {
  pub fn create(gl: WebGl2RenderingContext, program: WebGlProgram) -> Self {
    WebRenderAPI { gl, program }
  }
}

/**
 * The Web Gl binding for the interface.
 */
impl RenderAPI for WebRenderAPI {
  type Buffer = WebRenderBuffer;
  type UniformIndex = WebGlUniformLocation;

  fn bind_buffer<V>(
      &self,
      buffer: &Self::Buffer,
      view: &V,
      draw_kind: DrawKind,
  ) where V: View {
    let kind = buffer.buffer_kind_constant();
    let draw = draw_kind.draw_kind_constant();
    self.gl.bind_buffer(kind, Some(&buffer.internal));
    self.gl.buffer_data_with_array_buffer_view(kind, view.object(), draw)
  }

  fn blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
    self.gl.blend_color(red, green, blue, alpha);
  }

  fn blend_func(&self, src: BlendFuncFactor, dst: BlendFuncFactor) {
    self.gl.blend_func(src.blend_func_factor_constant(), dst.blend_func_factor_constant());
  }

  fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
    self.gl.clear_color(red, green, blue, alpha);
  }

  fn clear(&self, mask: ClearMask) {
    self.gl.clear(mask.clear_mask_constant());
  }

  fn create_buffer(&self, kind: BufferKind) -> Result<Self::Buffer, RenderApiError> {
    self.gl.create_buffer().ok_or(RenderApiError::FailedToCreateBuffer).map(|internal| {
      WebRenderBuffer { kind, internal }
    })
  }

  fn draw_arrays(&self, mode: DrawArrayKind, first: i32, count: i32) {
    self.gl.draw_arrays(mode.draw_array_kind_constant(), first, count);
  }

  fn enable_vertex_attrib_array<A>(&self, key: A) -> Result<(), RenderApiError> where A: IntoAttributeIndex {
    key.with_context(self).map(|i| self.gl.enable_vertex_attrib_array(i))
  }

  fn get_attribute<AK>(&self, key: AK) -> Result<AttributeIndex, RenderApiError> where AK: AttributeKey {
    let name = key.name();
    let glint = self.gl.get_attrib_location(&self.program, name);
    u32::try_from(glint).map_err(|_| RenderApiError::InvalidAttributeName(name.to_string()))
  }

  fn get_uniform<UK>(&self, key: UK) -> Result<Self::UniformIndex, RenderApiError> where UK: UniformKey {
    let name = key.name();
    let location = self.gl.get_uniform_location(&self.program, name);
    location.ok_or(RenderApiError::InvalidUniformName(name.to_string()))
  }

  fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32) {
    self.gl.viewport(x, y, width, height);
  }

  fn uniform2f<U>(&self, key: U, x: f32, y: f32) -> Result<(), RenderApiError> where U: IntoUniformIndex {
    key.with_context(self).map(|index| self.gl.uniform2f(Some(&index), x, y))
  }

  fn vertex_attrib_pointer_with_i32<A>(
      &self,
      key: A,
      size: i32,
      precision: ViewPrecision,
      normalized: bool,
      stride: i32,
      offset: i32
  ) -> Result<(), RenderApiError> where A: IntoAttributeIndex {
    key.with_context(self).map(|index| {
      self.gl.vertex_attrib_pointer_with_i32(
          index,
          size,
          precision.view_precision_constant(),
          normalized,
          stride,
          offset,
      )
    })
  }
}

#[derive(Clone, Debug)]
pub struct WebRenderBuffer {
  pub kind: BufferKind,
  pub internal: WebGlBuffer,
}

impl HasBufferKind for WebRenderBuffer {
  fn buffer_kind_constant(&self) -> u32 {
    self.kind.buffer_kind_constant()
  }
}

pub enum RenderApiError {
  FailedToCreateBuffer,
  InvalidAttributeName(String),
  InvalidUniformName(String),
}

impl ToString for RenderApiError {
  fn to_string(&self) -> String {
    match self {
      RenderApiError::FailedToCreateBuffer => "Failed to create buffer".to_string(),
      RenderApiError::InvalidAttributeName(s) => format!("Invalid attribute name, {}", s),
      RenderApiError::InvalidUniformName(s) => format!("Invalid uniform name, {}", s),
    }
  }
}

impl<A> IntoAttributeIndex for A where A: AttributeKey {
  fn with_context<C>(self, context: &C) -> Result<AttributeIndex, RenderApiError> where C: RenderAPI {
    context.get_attribute(self)
  }
}

impl IntoAttributeIndex for AttributeIndex {
  fn with_context<C>(self, _: &C) -> Result<AttributeIndex, RenderApiError> where C: RenderAPI {
    Ok(self)
  }
}

impl<U> IntoUniformIndex for U where U: UniformKey {
  fn with_context<C>(self, context: &C) -> Result<C::UniformIndex, RenderApiError> where C: RenderAPI {
    context.get_uniform(self)
  }
}
