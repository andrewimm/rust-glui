use glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent};
use glutin::dpi::{LogicalSize};
use glutin::ContextTrait;
use gllite;
use gllite::gli;
use gllite::texture::Texture;
use gllite::uniforms::UniformValue;
use gl;
use std::rc::Rc;
use std::thread;
use std::time::{self, SystemTime};

fn main() {
  let mut events_loop = EventsLoop::new();
  let wb = WindowBuilder::new()
    .with_title("Demo")
    .with_resizable(false)
    .with_dimensions(LogicalSize::from((400, 400)));
  let context = ContextBuilder::new()
    .with_vsync(true)
    .build_windowed(wb, &events_loop)
    .unwrap();

  unsafe {
    context.make_current().unwrap();
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
  }

  gllite::gli::clear_color(0.0, 0.0, 0.0, 1.0);

  let shader_frag = "#version 330
precision mediump float;
out vec4 outColor;

in vec2 v_position;

uniform vec4 color;
uniform sampler2D tex;

void main() {
  outColor = color * texture(tex, v_position);
}";
  let shader_vert = "#version 330
in vec2 a_position;
out vec2 v_position;

void main() {
  v_position = a_position;
  gl_Position = vec4(a_position.xy, 0, 1);
}";
  let mut prog = gllite::program::Program::new();
  prog
    .add_shader(shader_vert, gl::VERTEX_SHADER)
    .add_shader(shader_frag, gl::FRAGMENT_SHADER)
    .compile();

  let p = Rc::new(prog);

  let mut node = gllite::node::Node::for_program(Rc::clone(&p));

  let vertices: [f32; 6] = [
    0.0, 1.0,
    -1.0, -1.0,
    1.0, -1.0,
  ];

  node.add_attribute(String::from("a_position"));
  node.buffer_data(&vertices);
  node.set_uniform(String::from("color"), UniformValue::FloatVec4(1.0, 1.0, 0.0, 1.0));

  let tex = Texture::new();
  let check: [u8;16] = [
    30, 30, 30, 255,
    200, 200, 200, 255,
    200, 200, 200, 255,
    30, 30, 30, 255,
  ];
  tex.set_from_bytes(gli::RGBA, 2, 2, gli::RGBA, &check);
  tex.set_wrap_mode(gli::REPEAT, gli::REPEAT);
  tex.set_filter_mode(gli::NEAREST, gli::NEAREST);
  node.set_uniform(String::from("tex"), tex.as_uniform_value());

  let mut last_frame_time = SystemTime::now();
  loop {
    let now = SystemTime::now();
    let delta = match now.duration_since(last_frame_time) {
      Ok(n) => n.as_millis(),
      Err(_) => 1,
    };
    last_frame_time = now;

    if delta < 16 {
      let diff = 16 - delta;
      let sleeptime = time::Duration::from_millis(diff as u64);
      thread::sleep(sleeptime);
    }

    let mut should_exit = false;
    events_loop.poll_events(|event| {
      match event {
        Event::WindowEvent {event, ..} => match event {
          WindowEvent::CloseRequested => should_exit = true,
          _ => (),
        },
        _ => (),
      }
    });
    if should_exit {
      break;
    }

    p.make_current();
    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
      node.draw();
    }
    context.swap_buffers().unwrap();
  }
}