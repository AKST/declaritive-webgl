use web_sys::{WebGl2RenderingContext};

#[derive(Clone, Copy, Debug)]
pub enum DrawKind {
  StaticDraw,
  DynamicDraw,
  StreamDraw,
}

pub trait HasDrawKind {
  fn draw_kind_constant(&self) -> u32;
}

impl HasDrawKind for DrawKind {
  fn draw_kind_constant(&self) -> u32 {
    match self {
      DrawKind::StaticDraw => WebGl2RenderingContext::STATIC_DRAW,
      DrawKind::DynamicDraw => WebGl2RenderingContext::DYNAMIC_DRAW,
      DrawKind::StreamDraw => WebGl2RenderingContext::STREAM_DRAW,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum DrawArrayKind {
  Points,
  LineStrip,
  LineLoop,
  Lines,
  TriangleStrip,
  TriangleFan,
  Triangles,
}

pub trait HasDrawArrayKind {
  fn draw_array_kind_constant(&self) -> u32;
}

impl HasDrawArrayKind for DrawArrayKind {
  fn draw_array_kind_constant(&self) -> u32 {
    match self {
      DrawArrayKind::Points => WebGl2RenderingContext::POINTS,
      DrawArrayKind::LineStrip => WebGl2RenderingContext::LINE_STRIP,
      DrawArrayKind::LineLoop => WebGl2RenderingContext::LINE_LOOP,
      DrawArrayKind::Lines => WebGl2RenderingContext::LINES,
      DrawArrayKind::TriangleStrip => WebGl2RenderingContext::TRIANGLE_STRIP,
      DrawArrayKind::TriangleFan => WebGl2RenderingContext::TRIANGLE_FAN,
      DrawArrayKind::Triangles => WebGl2RenderingContext::TRIANGLES,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum ClearMask {
  ColorBufferBit,
  DepthBufferBit,
  StencilBufferBit,
}

pub trait HasClearMaskKind {
  fn clear_mask_constant(&self) -> u32;
}

impl HasClearMaskKind for ClearMask {
  fn clear_mask_constant(&self) -> u32 {
    match self {
      ClearMask::ColorBufferBit => WebGl2RenderingContext::COLOR_BUFFER_BIT,
      ClearMask::DepthBufferBit => WebGl2RenderingContext::DEPTH_BUFFER_BIT,
      ClearMask::StencilBufferBit => WebGl2RenderingContext::STENCIL_BUFFER_BIT,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum BufferKind {
  ArrayBuffer,
  ElementBuffer,
}

pub trait HasBufferKind {
  fn buffer_kind_constant(&self) -> u32;
}

impl HasBufferKind for BufferKind {
  fn buffer_kind_constant(&self) -> u32 {
    match self {
      BufferKind::ArrayBuffer => WebGl2RenderingContext::ARRAY_BUFFER,
      BufferKind::ElementBuffer => WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER_BINDING,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum ViewPrecision {
  Byte,
  Short,
  UnsignedByte,
  UnsignedShort,
  Float,
}

pub trait HasViewPrecision {
  fn view_precision_constant(&self) -> u32;
}

impl HasViewPrecision for ViewPrecision {
  fn view_precision_constant(&self) -> u32 {
    match self {
      ViewPrecision::Byte => WebGl2RenderingContext::BYTE,
      ViewPrecision::Short => WebGl2RenderingContext::SHORT,
      ViewPrecision::UnsignedByte => WebGl2RenderingContext::UNSIGNED_BYTE,
      ViewPrecision::UnsignedShort => WebGl2RenderingContext::UNSIGNED_SHORT,
      ViewPrecision::Float => WebGl2RenderingContext::FLOAT,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum BlendFuncFactor {
  /**
   * Multiplies all colors by 0.
   */
  Zero,
  /**
   * Multiplies all colors by 1.
   */
  One,
  /**
   * Multiplies all colors by the source colors.
   */
  SrcColor,
  /**
   * Multiplies all colors by 1 minus each source color.
   */
  OneMinusSrcColor,
  /**
   * Multiplies all colors by the destination color.
   */
  DstColor,
  /**
   * Multiplies all colors by 1 minus each destination color.
   */
  OneMinusDstColor,
  /**
   * Multiplies all colors by the source alpha value.
   */
  SrcAlpha,
  /**
   * Multiplies all colors by 1 minus the source alpha value.
   */
  OneMinusSrcAlpha,
  /**
   * Multiplies all colors by the destination alpha value.
   */
  DstAlpha,
  /**
   * Multiplies all colors by 1 minus the destination alpha value.
   */
  OneMinusDstAlpha,
  /**
   * Multiplies all colors by the destination alpha value.
   */
  ConstantColor,
  /**
   * Multiplies all colors by 1 minus the destination alpha value.
   */
  OneMinusConstantColor,
  /**
   * Multiplies all colors by a constant color.
   */
  ConstantAlpha,
  /**
   * Multiplies all colors by 1 minus a constant alpha value.
   */
  OneMinusConstantAlpha,
  /**
   * Multiplies the RGB colors by the smaller of either the
   * source alpha value or the value of 1 minus the destination
   * alpha value. The alpha value is multiplied by 1.
   */
  SrcAlphaSaturate,
}

pub trait HasBlendFuncFactor {
  fn blend_func_factor_constant(&self) -> u32;
}

impl HasBlendFuncFactor for BlendFuncFactor {
  fn blend_func_factor_constant(&self) -> u32 {
    match self {
      BlendFuncFactor::One => WebGl2RenderingContext::ONE,
      BlendFuncFactor::Zero => WebGl2RenderingContext::ZERO,
      BlendFuncFactor::SrcColor => WebGl2RenderingContext::SRC_COLOR,
      BlendFuncFactor::OneMinusSrcColor => WebGl2RenderingContext::ONE_MINUS_SRC_COLOR,
      BlendFuncFactor::DstColor => WebGl2RenderingContext::DST_COLOR,
      BlendFuncFactor::OneMinusDstColor => WebGl2RenderingContext::ONE_MINUS_DST_COLOR,
      BlendFuncFactor::SrcAlpha => WebGl2RenderingContext::SRC_ALPHA,
      BlendFuncFactor::OneMinusSrcAlpha => WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
      BlendFuncFactor::DstAlpha => WebGl2RenderingContext::DST_ALPHA,
      BlendFuncFactor::OneMinusDstAlpha => WebGl2RenderingContext::ONE_MINUS_DST_ALPHA,
      BlendFuncFactor::ConstantColor => WebGl2RenderingContext::CONSTANT_COLOR,
      BlendFuncFactor::OneMinusConstantColor => WebGl2RenderingContext::ONE_MINUS_CONSTANT_COLOR,
      BlendFuncFactor::ConstantAlpha => WebGl2RenderingContext::CONSTANT_ALPHA,
      BlendFuncFactor::OneMinusConstantAlpha => WebGl2RenderingContext::ONE_MINUS_CONSTANT_ALPHA,
      BlendFuncFactor::SrcAlphaSaturate => WebGl2RenderingContext::SRC_ALPHA_SATURATE,
    }
  }
}
