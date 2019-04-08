use super::gli;

pub enum UniformValue {
  Float(f32),
  FloatVec2(f32, f32),
  FloatVec3(f32, f32, f32),
  FloatVec4(f32, f32, f32, f32),

  Texture2D(u32),
}

pub fn set_value_for_uniform(location: u32, value: &UniformValue) {
  match value {
    UniformValue::Float(f) => gli::uniform_1f(location, *f),
    UniformValue::FloatVec2(x, y) => gli::uniform_2f(location, *x, *y),
    UniformValue::FloatVec3(x, y, z) => gli::uniform_3f(location, *x, *y, *z),
    UniformValue::FloatVec4(x, y, z, w) => gli::uniform_4f(location, *x, *y, *z, *w),
    _ => (),
  }
}
