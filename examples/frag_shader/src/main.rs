use trivalibs::{
	bmap,
	painter::{
		create_canvas_app,
		effect::EffectProps,
		layer::{Layer, LayerProps},
		load_fragment_shader,
		painter::UniformType,
		shade::ShadeEffectProps,
		uniform::UniformBuffer,
		wgpu::SurfaceError,
		winit::event::{DeviceEvent, WindowEvent},
		CanvasApp, Painter,
	},
	prelude::*,
};

struct RenderState {
	time: UniformBuffer<f32>,
	size: UniformBuffer<UVec2>,
	canvas: Layer,
}

#[derive(Default)]
struct App {
	time: f32,
}

impl CanvasApp<RenderState, ()> for App {
	fn init(&self, p: &mut Painter) -> RenderState {
		let u_type = p.uniform_type_buffered_frag();

		let shade = p.shade_create_effect(ShadeEffectProps {
			uniform_types: &[&u_type, &u_type],
		});
		load_fragment_shader!(shade, p, "../shader/main.spv");

		let time = u_type.create_buff(p, 0.0f32);
		let size = u_type.create_buff(p, uvec2(0, 0));

		let effect = p.effect_create(
			shade,
			&EffectProps {
				uniforms: bmap! {
					0 => size.uniform,
					1 => time.uniform,
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
