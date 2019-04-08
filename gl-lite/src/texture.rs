use super::gli;
use super::uniforms::UniformValue;

pub struct Texture {
  gl_texture: u32,
  width: i32,
  height: i32,
}

impl Texture {
  pub fn new() -> Texture {
    let tex = gli::create_texture();
    gli::bind_texture_2d(tex);
    gli::tex_parameter_2d(gli::TEXTURE_WRAP_S, gli::CLAMP_TO_EDGE);
    gli::tex_parameter_2d(gli::TEXTURE_WRAP_T, gli::CLAMP_TO_EDGE);
    gli::tex_parameter_2d(gli::TEXTURE_MIN_FILTER, gli::LINEAR);
    gli::tex_parameter_2d(gli::TEXTURE_MAG_FILTER, gli::LINEAR);
    let empty: [u8; 4] = [0, 0, 0, 0];
    gli::tex_image_2d_from_bytes(gli::RGBA, 1, 1, gli::RGBA, &empty);
    Texture {
      gl_texture: tex,
      width: 1,
      height: 1,
    }
  }

  pub fn set_wrap_mode(&self, s: u32, t: u32) {
    gli::bind_texture_2d(self.gl_texture);
    gli::tex_parameter_2d(gli::TEXTURE_WRAP_S, s);
    gli::tex_parameter_2d(gli::TEXTURE_WRAP_T, t);
  }

  pub fn set_filter_mode(&self, min: u32, mag: u32) {
    gli::bind_texture_2d(self.gl_texture);
    gli::tex_parameter_2d(gli::TEXTURE_MIN_FILTER, min);
    gli::tex_parameter_2d(gli::TEXTURE_MAG_FILTER, mag);
  }

  pub fn as_uniform_value(&self) -> UniformValue {
    UniformValue::Texture2D(self.gl_texture)
  }

  pub fn set_from_bytes(&self, internal: u32, width: i32, height: i32, format: u32, data: &[u8]) {
    gli::bind_texture_2d(self.gl_texture);
    gli::tex_image_2d_from_bytes(internal, width, height, format, data);
  }

  pub fn bind_to_slot(&self, slot: u32) {
    gli::active_texture(slot);
    gli::bind_texture_2d(self.gl_texture);
  }
}
