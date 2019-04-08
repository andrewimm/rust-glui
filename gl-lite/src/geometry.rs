use super::gli;
use super::program::Attribute;

pub struct AttributeInfo {
  normalize: bool,
  location: u32,
  offset: i32,
  size: i32,
  stride: i32,
  gl_type: u32,
}

pub struct Geometry {
  attributes: Vec<AttributeInfo>,
  buffer: VertexBuffer,
  data_length: i32,
  index: Option<u32>,
  index_count: i32,
  total_byte_length: i32,
  vao: u32,
  bound: bool,
}

impl Geometry {
  pub fn new() -> Geometry {
    Geometry {
      attributes: Vec::new(),
      buffer: VertexBuffer::new(),
      data_length: 0,
      index: None,
      index_count: 0,
      total_byte_length: 0,
      vao: gli::create_vertex_array(),
      bound: false,
    }
  }

  fn add_attr(&mut self, attr: &Attribute, normalize: bool) {
    let (size, gl_type) = gli::get_attribute_size_and_type(attr.gl_type, normalize);
    let length = gli::size_of_type(gl_type);
    let info = AttributeInfo {
      normalize: normalize,
      location: attr.location,
      size: size,
      gl_type: gl_type,
      offset: self.total_byte_length,
      stride: length * size,
    };
    self.attributes.push(info);
    self.total_byte_length += length * size;
  }

  pub fn add_attribute(&mut self, attr: &Attribute) {
    self.add_attr(attr, false);
  }

  pub fn add_normalized_attribute(&mut self, attr: &Attribute) {
    self.add_attr(attr, true);
  }

  pub fn buffer_data(&mut self, data: &[f32]) {
    self.data_length = data.len() as i32 * 4;
    self.buffer.buffer_data(data);
  }

  fn bind_to_attributes(&self) {
    for attr in self.attributes.iter() {
      self.buffer.bind_to_attribute(
        attr.location,
        attr.size,
        attr.gl_type,
        attr.normalize,
        self.total_byte_length,
        attr.offset,
      );
    }
  }

  pub fn draw(&mut self) {
    gli::bind_vertex_array(self.vao);
    if !self.bound {
      self.bind_to_attributes();
      self.bound = true;
    }
    if let Some(index) = self.index {
      gli::draw_elements_triangles(self.index_count);
    } else {
      let count = self.data_length / self.total_byte_length;
      gli::draw_arrays_triangles(count);
    }
    gli::bind_vertex_array(0);
  }
}

pub struct VertexBuffer {
  buffer: u32,
}

impl VertexBuffer {
  pub fn new() -> VertexBuffer {
    let vbo = gli::create_buffer();
    VertexBuffer {
      buffer: vbo,
    }
  }

  pub fn buffer_data(&self, data: &[f32]) {
    gli::bind_array_buffer(self.buffer);
    gli::buffer_array_data(data);
  }

  pub fn bind_to_attribute(&self, location: u32, size: i32, gl_type: u32, normalized: bool, stride: i32, offset: i32) {
    gli::bind_array_buffer(self.buffer);
    gli::vertex_attrib_pointer(location, size, gl_type, normalized, stride, offset);
    gli::enable_vertex_attrib_array(location);
  }
}
