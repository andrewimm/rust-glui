use std::collections::HashMap;
use std::rc::Rc;
use super::geometry::Geometry;
use super::gli;
use super::program::{Program};
use super::uniforms::{UniformValue, set_value_for_uniform};

type LocalUniformMap = HashMap<String, UniformValue>;

pub struct Node {
  geometry: Geometry,
  program: Rc<Program>,
  uniforms: LocalUniformMap,
}

impl Node {
  pub fn for_program(program: Rc<Program>) -> Node {
    Node {
      geometry: Geometry::new(),
      program: program,
      uniforms: HashMap::new(),
    }
  }

  pub fn add_attribute(&mut self, name: String) {
    if let Some(attr) = self.program.get_attribute(&name) {
      self.geometry.add_attribute(attr);
    }
  }

  pub fn buffer_data(&mut self, data: &[f32]) {
    self.geometry.buffer_data(data);
  }

  pub fn set_uniform(&mut self, name: String, value: UniformValue) {
    self.uniforms.insert(name, value);
  }

  pub fn draw(&mut self) {
    let mut tex_slot = 0;
    for (name, uniform) in self.program.uniforms.iter() {
      if let Some(local) = self.uniforms.get(name) {
        if let UniformValue::Texture2D(t) = local {
          gli::active_texture(tex_slot);
          gli::bind_texture_2d(*t);
          gli::uniform_1i(uniform.location, tex_slot as i32);
          tex_slot += 1;
        } else {
          set_value_for_uniform(uniform.location, &local);
        }
      }
    }

    self.geometry.draw();
  }
}