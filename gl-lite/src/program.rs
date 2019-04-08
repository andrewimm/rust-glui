use std::collections::HashMap;
use super::gli;

pub struct Attribute {
  pub location: u32,
  pub size: i32,
  pub gl_type: u32,
}

pub struct Uniform {
  pub location: u32,
  pub size: i32,
  pub gl_type: u32,
}

type AttributeMap = HashMap<String, Attribute>;

type UniformMap = HashMap<String, Uniform>;

type RawShader = (u32, &'static str);

pub struct Program {
  pub attributes: AttributeMap,
  pub uniforms: UniformMap,
  program: Option<u32>,
  raw_shaders: Vec<RawShader>,
}

impl Program {
  pub fn new() -> Program {
    Program {
      attributes: HashMap::new(),
      uniforms: HashMap::new(),
      program: None,
      raw_shaders: Vec::new(),
    }
  }

  pub fn add_shader(&mut self, source: &'static str, shader_type: u32) -> &mut Program {
    if let Some(_) = self.program {
      panic!("Cannot add shader, the program has already been compiled");
    }
    self.raw_shaders.push((shader_type, source));
    self
  }

  pub fn compile(&mut self) -> &mut Program {
    if let Some(_) = self.program {
      panic!("Cannot compile, the program has already been compiled")
    }
    let raw = &mut self.raw_shaders;
    let compiled: Vec<u32> = raw.into_iter().map(|raw_shader| {
      let (shader_type, source) = raw_shader;
      let shader = gli::create_shader(*shader_type);
      gli::shader_source(shader, source);
      gli::compile_shader(shader);
      shader
    }).collect();

    let program = gli::create_program();
    for shader in compiled {
      gli::attach_shader(program, shader);
    }
    gli::link_program(program);
    self.program = Some(program);
    extract_uniforms(program, &mut self.uniforms);
    extract_attributes(program, &mut self.attributes);
    self
  }

  pub fn make_current(&self) {
    if let Some(p) = self.program {
      gli::use_program(p);
    }
  }

  pub fn get_attribute(&self, name: &String) -> Option<&Attribute> {
    self.attributes.get(name)
  }
}

fn extract_uniforms(program: u32, map: &mut UniformMap) {
  let count = gli::get_active_uniform_count(program);
  for i in 0..count {
    let (name, size, uniform_type) = gli::get_active_uniform(program, i);
    let location = gli::get_uniform_location(program, name.as_str());
    if location > -1 {
      let uniform = Uniform {
        location: location as u32,
        size: size,
        gl_type: uniform_type,
      };
      map.insert(name, uniform);
    }
  }
}

fn extract_attributes(program: u32, map: &mut AttributeMap) {
  let count = gli::get_active_attribute_count(program);
  for i in 0..count {
    let (name, size, uniform_type) = gli::get_active_attribute(program, i);
    let location = gli::get_attribute_location(program, name.as_str());
    if location > -1 {
      let attrib = Attribute {
        location: location as u32,
        size: size,
        gl_type: uniform_type,
      };
      map.insert(name, attrib);
    }
  }
}