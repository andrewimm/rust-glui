use gl;
use gl::types::{GLint, GLfloat, GLsizeiptr};
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub fn init() {

}

pub fn create_shader(shader_type: u32) -> u32 {
  unsafe {
    gl::CreateShader(shader_type)
  }
}

pub fn shader_source(shader: u32, source: &str) {
  unsafe {
    let cstr = CString::new(source.as_bytes()).unwrap();
    gl::ShaderSource(shader, 1, &cstr.as_ptr(), ptr::null());
  }
}

pub fn compile_shader(shader: u32) {
  unsafe {
    gl::CompileShader(shader);

    let mut success = gl::FALSE as GLint;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
      let mut bytes: [i8; 512] = [0;512];
      gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), &mut bytes[0] as *mut i8);
      let u8bytes = &*(&bytes[..] as *const [i8] as *const [u8]);
      gl::DeleteShader(shader);
      panic!("Failed to compile shader: {}", std::str::from_utf8(u8bytes).unwrap());
    }
  }
}

pub fn create_program() -> u32 {
  unsafe {
    gl::CreateProgram()
  }
}

pub fn attach_shader(program: u32, shader: u32) {
  unsafe {
    gl::AttachShader(program, shader);
  }
}

pub fn link_program(program: u32) {
  unsafe {
    gl::LinkProgram(program);

    let mut success = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
      let mut bytes: [i8; 512] = [0;512];
      gl::GetProgramInfoLog(program, 512, ptr::null_mut(), &mut bytes[0] as *mut i8);
      let u8bytes = &*(&bytes[..] as *const [i8] as *const [u8]);
      gl::DeleteProgram(program);
      panic!("Failed to link program: {}", std::str::from_utf8(u8bytes).unwrap());
    }
  }
}

pub fn use_program(program: u32) {
  unsafe {
    gl::UseProgram(program);
  }
}

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
  unsafe {
    gl::ClearColor(r, g, b, a);
  }
}

pub fn get_active_uniform_count(program: u32) -> u32 {
  let mut count: i32 = 0;
  unsafe {
    gl::GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut count);
  }
  return count as u32;
}

pub fn get_active_uniform(program: u32, index: u32) -> (String, i32, u32) {
  unsafe {
    let mut buf: [i8; 128] = [0;128];
    let mut name_length: i32 = 0;
    let mut size: i32 = 0;
    let mut uniform_type: u32 = 0;
    gl::GetActiveUniform(program, index, 128, &mut name_length, &mut size, &mut uniform_type, &mut buf[0] as *mut i8);
    let mut bytes: Vec<u8> = Vec::with_capacity(name_length as usize);
    for i in 0..name_length {
      bytes.push(buf[i as usize] as u8);
    }
    let name = String::from_utf8(bytes).unwrap();
    (name, size, uniform_type)
  }
}

pub fn get_uniform_location(program: u32, name: &str) -> i32 {
  unsafe {
    gl::GetUniformLocation(
      program,
      CString::new(name).unwrap().as_ptr()
    )
  }
}

pub fn get_active_attribute_count(program: u32) -> u32 {
  let mut count: i32 = 0;
  unsafe {
    gl::GetProgramiv(program, gl::ACTIVE_ATTRIBUTES, &mut count);
  }
  return count as u32;
}

pub fn get_active_attribute(program: u32, index: u32) -> (String, i32, u32) {
  unsafe {
    let mut buf: [i8; 128] = [0;128];
    let mut name_length: i32 = 0;
    let mut size: i32 = 0;
    let mut uniform_type: u32 = 0;
    gl::GetActiveAttrib(program, index, 128, &mut name_length, &mut size, &mut uniform_type, &mut buf[0] as *mut i8);
    let mut bytes: Vec<u8> = Vec::with_capacity(name_length as usize);
    for i in 0..name_length {
      bytes.push(buf[i as usize] as u8);
    }
    let name = String::from_utf8(bytes).unwrap();
    (name, size, uniform_type)
  }
}

pub fn get_attribute_location(program: u32, name: &str) -> i32 {
  unsafe {
    gl::GetAttribLocation(
      program,
      CString::new(name).unwrap().as_ptr()
    )
  }
}

pub fn get_attribute_size_and_type(attr_type: u32, normalize: bool) -> (i32, u32) {
  let t = if normalize { gl::UNSIGNED_BYTE } else { gl::FLOAT };
  match attr_type {
    gl::FLOAT => (1, t),
    gl::FLOAT_VEC2 => (2, t),
    gl::FLOAT_VEC3 => (3, t),
    gl::FLOAT_VEC4 => (4, t),
    gl::FLOAT_MAT2 => (4, t),
    gl::FLOAT_MAT3 => (9, t),
    gl::FLOAT_MAT4 => (16, t),
    _ => (1, t),
  }
}

pub fn size_of_type(gl_type: u32) -> i32 {
  match gl_type {
    gl::BYTE => 1,
    gl::UNSIGNED_BYTE => 1,
    gl::SHORT => 2,
    gl::UNSIGNED_SHORT => 2,
    gl::FLOAT => 4,
    _ => 1,
  }
}

pub fn create_vertex_array() -> u32 {
  unsafe {
    let mut vao = 0;
    gl::GenVertexArrays(1, &mut vao);
    vao
  }
}

pub fn bind_vertex_array(array: u32) {
  unsafe {
    gl::BindVertexArray(array);
  }
}

pub fn create_buffer() -> u32 {
  unsafe {
    let mut vbo = 0;
    gl::GenBuffers(1, &mut vbo);
    vbo
  }
}

pub fn bind_array_buffer(buffer: u32) {
  unsafe {
    gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
  }
}

pub fn bind_element_array_buffer(buffer: u32) {
  unsafe {
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer);
  }
}

pub fn draw_arrays_triangles(count: i32) {
  unsafe {
    gl::DrawArrays(gl::TRIANGLES, 0, count);
  }
}

pub fn draw_elements_triangles(count: i32) {
  unsafe {
    gl::DrawElements(gl::TRIANGLES, count, gl::UNSIGNED_SHORT, ptr::null());
  }
}

pub fn buffer_array_data(data: &[f32]) {
  let float_size = mem::size_of::<GLfloat>();
  unsafe {
    gl::BufferData(
      gl::ARRAY_BUFFER,
      (data.len() * float_size) as GLsizeiptr,
      &data[0] as *const f32 as *const c_void,
      gl::STATIC_DRAW
    );
  }
}

pub fn vertex_attrib_pointer(location: u32, size: i32, gl_type: u32, normalized: bool, stride: i32, offset: i32) {
  unsafe {
    let normalized_value = if normalized { gl::TRUE } else { gl::FALSE };
    gl::VertexAttribPointer(location, size, gl_type, normalized_value, stride, ptr::null());
  }
}

pub fn enable_vertex_attrib_array(location: u32) {
  unsafe {
    gl::EnableVertexAttribArray(location);
  }
}

pub fn uniform_1f(location: u32, f: f32) {
  unsafe {
    gl::Uniform1f(location as i32, f);
  }
}

pub fn uniform_2f(location: u32, x: f32, y: f32) {
  unsafe {
    gl::Uniform2f(location as i32, x, y);
  }
}

pub fn uniform_3f(location: u32, x: f32, y: f32, z: f32) {
  unsafe {
    gl::Uniform3f(location as i32, x, y, z);
  }
}

pub fn uniform_4f(location: u32, x: f32, y: f32, z: f32, w: f32) {
  unsafe {
    gl::Uniform4f(location as i32, x, y, z, w);
  }
}

pub fn uniform_1i(location: u32, i: i32) {
  unsafe {
    gl::Uniform1i(location as i32, i);
  }
}

pub fn uniform_2i(location: u32, x: i32, y: i32) {
  unsafe {
    gl::Uniform2i(location as i32, x, y);
  }
}

pub fn uniform_3i(location: u32, x: i32, y: i32, z: i32) {
  unsafe {
    gl::Uniform3i(location as i32, x, y, z);
  }
}

pub fn uniform_4i(location: u32, x: i32, y: i32, z: i32, w: i32) {
  unsafe {
    gl::Uniform4i(location as i32, x, y, z, w);
  }
}

pub fn create_texture() -> u32 {
  unsafe {
    let mut t = 0;
    gl::GenTextures(1, &mut t);
    t
  }
}

pub fn active_texture(unit: u32) {
  unsafe {
    gl::ActiveTexture(gl::TEXTURE0 + unit);
  }
}

pub fn bind_texture_2d(unit: u32) {
  unsafe {
    gl::BindTexture(gl::TEXTURE_2D, unit);
  }
}

pub fn tex_parameter_2d(param: u32, value: u32) {
  unsafe {
    gl::TexParameteri(gl::TEXTURE_2D, param, value as i32);
  }
}

pub fn tex_image_2d_from_bytes(internal_format: u32, width: i32, height: i32, format: u32, data: &[u8]) {
  unsafe {
    gl::TexImage2D(gl::TEXTURE_2D, 0, internal_format as i32, width, height, 0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
  }
}

pub use gl::TEXTURE_MIN_FILTER;
pub use gl::TEXTURE_MAG_FILTER;
pub use gl::TEXTURE_WRAP_S;
pub use gl::TEXTURE_WRAP_T;

pub use gl::NEAREST;
pub use gl::LINEAR;
pub use gl::CLAMP_TO_EDGE;
pub use gl::REPEAT;

pub use gl::R8UI;
pub use gl::RGB;
pub use gl::RGBA;
pub use gl::RED_INTEGER;
