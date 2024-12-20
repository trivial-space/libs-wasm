use trivalibs::{
	bmap,
	painter::{
		create_canvas_app,
		layer::{Layer, LayerProps},
		load_fragment_shader, load_vertex_shader,
		shade::ShadeProps,
		sketch::SketchProps,
		wgpu::{SurfaceError, VertexFormat::*},
		winit::event::{DeviceEvent, WindowEvent},
		CanvasApp, Painter, UniformType,
	},
	prelude::*,
	rendering::{camera::PerspectiveCamera, transform::Transform},
};

#[apply(gpu_data)]
struct Vertex {
	pos: Vec3,
	uv: Vec2,
}

const TRIANGLE: [Vertex; 3] = [
	Vertex {
		pos: vec3(-1.0, -1.0, 0.0),
		uv: vec2(0.0, 0.0),
	},
	Vertex {
		pos: vec3(1.0, -1.0, 0.0),
		uv: vec2(1.0, 0.0),
	},
	Vertex {
		pos: vec3(0.0, 1.0, 0.0),
		uv: vec2(0.5, 1.0),
	},
];

const QUAD: [Vertex; 6] = [
	Vertex {
		pos: vec3(-1.0, -1.0, 0.0),
		uv: vec2(0.0, 0.0),
	},
	Vertex {
		pos: vec3(1.0, -1.0, 0.0),
		uv: vec2(1.0, 0.0),
	},
	Vertex {
		pos: vec3(-1.0, 1.0, 0.0),
		uv: vec2(0.0, 1.0),
	},
	Vertex {
		pos: vec3(-1.0, 1.0, 0.0),
		uv: vec2(0.0, 1.0),
	},
	Vertex {
		pos: vec3(1.0, -1.0, 0.0),
		uv: vec2(1.0, 0.0),
	},
	Vertex {
		pos: vec3(1.0, 1.0, 0.0),
		uv: vec2(1.0, 1.0),
	},
];

struct RenderState {
	red_triangle: Layer,
	blue_quad: Layer,
	canvas: Layer,
}

#[derive(Default)]
struct App {
	cam: PerspectiveCamera,
	triangle_transform: Transform,
	quad_transform: Transform,
}

impl CanvasApp<RenderState, ()> for App {
	fn init(&self, p: &mut Painter) -> RenderState {
		let u_fs_type = p.uniform_type_buffered_frag();
		let u_vs_type = p.uniform_type_buffered_vert();
		let tex_type = p.uniform_type_tex_2d_frag();

		let color_shade = p.shade_create(ShadeProps {
			uniform_types: &[&u_vs_type, &u_fs_type],
			vertex_format: &[Float32x3, Float32x2],
		});
		load_vertex_shader!(color_shade, p, "../color_shader/vs_main.spv");
		load_fragment_shader!(color_shade, p, "../color_shader/fs_main.spv");

		let tex_shader = p.shade_create(ShadeProps {
			uniform_types: &[&u_vs_type, &tex_type],
			vertex_format: &[Float32x3, Float32x2],
		});
		load_vertex_shader!(tex_shader, p, "../tex_shader/vs_main.spv");
		load_fragment_shader!(tex_shader, p, "../tex_shader/fs_main.spv");

		let quad_form = p.form_from_buffer(QUAD.as_slice(), default());
		let triangle_form = p.form_from_buffer(TRIANGLE.as_slice(), default());

		let quad_mvp = u_vs_type.create_buff(p, Mat4::IDENTITY);
		let triangle_mvp = u_vs_type.create_buff(p, Mat4::IDENTITY);

		let quad_color = vec3(0.0, 0.0, 1.0);
		let triangle_color = vec3(1.0, 0.0, 0.0);

		let color_quad_sketch = p.sketch_create(
			quad_form,
			color_shade,
			&SketchProps {
				uniforms: bmap! {
					0 => quad_mvp.uniform,
					1 => u_fs_type.const_vec3(p, quad_color),
				},
				..default()
			},
		);

		let color_triangle_sketch = p.sketch_create(
			triangle_form,
			color_shade,
			&SketchProps {
				uniforms: bmap! {
					0 => triangle_mvp.uniform,
					1 => u_fs_type.const_vec3(p, triangle_color),
				},
				..default()
			},
		);

		let canvas = p.layer_create(&LayerProps {
			effects: vec![effect],
			..default()
		});

		RenderState { canvas, time, size }
	}

	fn resize(&mut self, p: &mut Painter, rs: &mut RenderState) {
		let size = p.canvas_size();
		rs.size.update(p, uvec2(size.width, size.height));
	}

	fn update(&mut self, p: &mut Painter, rs: &mut RenderState, tpf: f32) {
		self.time += tpf;
		rs.time.update(p, self.time);
	}

	fn render(&self, p: &mut Painter, state: &RenderState) -> Result<(), SurfaceError> {
		p.paint(&state.canvas)?;
		p.show(&state.canvas)?;

		p.request_next_frame();

		Ok(())
	}

	fn user_event(&mut self, _e: (), _p: &Painter) {}
	fn window_event(&mut self, _e: WindowEvent, _p: &Painter) {}
	fn device_event(&mut self, _e: DeviceEvent, _p: &Painter) {}
}

pub fn main() {
	create_canvas_app(App::default()).start();
}