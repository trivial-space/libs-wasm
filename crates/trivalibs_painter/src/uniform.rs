use super::{painter::get_padded_size, Painter};
use trivalibs_core::glam::{Mat3, Mat3A, Vec3, Vec3A};

#[derive(Debug, Clone, Copy)]
pub struct Uniform(pub(crate) usize);

pub struct UniformBuffer<T> {
	pub uniform: Uniform,
	buffer: wgpu::Buffer,
	t: std::marker::PhantomData<T>,
}

pub fn get_uniform_layout_buffered(
	painter: &Painter,
	visibility: wgpu::ShaderStages,
) -> wgpu::BindGroupLayout {
	painter
		.device
		.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			entries: &[wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: None,
				},
				count: None,
			}],
			label: None,
		})
}

impl<T> UniformBuffer<T>
where
	T: bytemuck::Pod,
{
	pub fn new(painter: &mut Painter, layout: &wgpu::BindGroupLayout, data: T) -> Self {
		let buffer = painter.device.create_buffer(&wgpu::BufferDescriptor {
			label: None,
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			size: get_padded_size(std::mem::size_of::<T>() as u64),
			mapped_at_creation: false,
		});

		let bind_group = painter
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				layout,
				entries: &[wgpu::BindGroupEntry {
					binding: 0,
					resource: buffer.as_entire_binding(),
				}],
				label: None,
			});

		painter.bindings.push(bind_group);

		let binding = Uniform(painter.bindings.len() - 1);

		let uniform = UniformBuffer {
			buffer,
			uniform: binding,
			t: std::marker::PhantomData,
		};

		uniform.update(&painter, data);

		uniform
	}

	pub fn update(&self, painter: &Painter, data: T) {
		painter
			.queue
			.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[data]));
	}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable)]
pub struct Mat3U(Mat3A);
unsafe impl bytemuck::Pod for Mat3U {}

impl UniformBuffer<Mat3U> {
	pub fn new_mat3(painter: &mut Painter, layout: &wgpu::BindGroupLayout, data: Mat3) -> Self {
		UniformBuffer::new(painter, layout, Mat3U(Mat3A::from(data)))
	}

	pub fn update_mat3(&self, painter: &Painter, data: Mat3) {
		self.update(painter, Mat3U(Mat3A::from(data)));
	}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable)]
pub struct Vec3U(Vec3A);
unsafe impl bytemuck::Pod for Vec3U {}

impl UniformBuffer<Vec3U> {
	pub fn new_vec3(painter: &mut Painter, layout: &wgpu::BindGroupLayout, data: Vec3) -> Self {
		UniformBuffer::new(painter, layout, Vec3U(Vec3A::from(data)))
	}

	pub fn update_vec3(&self, painter: &Painter, data: Vec3) {
		self.update(painter, Vec3U(Vec3A::from(data)));
	}
}
