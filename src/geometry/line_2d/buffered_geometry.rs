use super::{Line, LineData};
use crate::{
    data_structures::neighbour_list::traits::WithNeighboursTransform,
    rendering::buffered_geometry::{
        attr_idx, create_buffered_geometry_layout, AttributeIndex, BufferedGeometry,
        BufferedGeometryAttribute, RenderingPrimitive, ToBufferedGeometry, VertexAttributeValue,
        VertexFormat::{Float32, Float32x2},
    },
    utils::default,
};
use glam::Vec2;

pub enum VertexAttribute {
    Position(Vec2),
    Width(f32),
    Length(f32),
    Uv(Vec2),
    LocalUv(Vec2),
}

static POS_ATTR: AttributeIndex = attr_idx("position", Float32x2, 0);
static WIDTH_ATTR: AttributeIndex = attr_idx("width", Float32, 1);
static LENGTH_ATTR: AttributeIndex = attr_idx("length", Float32, 2);
static UV_ATTR: AttributeIndex = attr_idx("uv", Float32x2, 3);
static LOCAL_UV_ATTR: AttributeIndex = attr_idx("local_uv", Float32x2, 4);

impl BufferedGeometryAttribute for VertexAttribute {
    fn attribute_index(&self) -> AttributeIndex {
        match self {
            VertexAttribute::Position(_) => POS_ATTR,
            VertexAttribute::Width(_) => WIDTH_ATTR,
            VertexAttribute::Length(_) => LENGTH_ATTR,
            VertexAttribute::Uv(_) => UV_ATTR,
            VertexAttribute::LocalUv(_) => LOCAL_UV_ATTR,
        }
    }

    fn attribute_value(&self) -> VertexAttributeValue {
        match self {
            VertexAttribute::Position(v) => VertexAttributeValue::Float32x2(v.to_array()),
            VertexAttribute::Width(v) => VertexAttributeValue::Float32(*v),
            VertexAttribute::Length(v) => VertexAttributeValue::Float32(*v),
            VertexAttribute::Uv(v) => VertexAttributeValue::Float32x2(v.to_array()),
            VertexAttribute::LocalUv(v) => VertexAttributeValue::Float32x2(v.to_array()),
        }
    }
}

pub struct LineGeometryOpts {
    pub total_length: Option<f32>,
    pub prev_direction: Option<Vec2>,
    pub next_direction: Option<Vec2>,

    pub smouth_depth: u8,
    pub smouth_angle_threshold: f32,
    pub smouth_min_length: f32,

    pub cap_width_length_ratio: f32,
    pub swap_texture_orientation: bool,
}

impl Default for LineGeometryOpts {
    fn default() -> Self {
        Self {
            smouth_depth: 2,
            smouth_min_length: 3.0,
            smouth_angle_threshold: 0.05,

            cap_width_length_ratio: 1.0,
            swap_texture_orientation: false,

            total_length: None,
            prev_direction: None,
            next_direction: None,
        }
    }
}

impl ToBufferedGeometry for Line {
    fn to_buffered_geometry(&self) -> BufferedGeometry {
        self.to_buffered_geometry_with(default())
    }
}

fn get_normal(direction: &Vec2) -> Vec2 {
    Vec2::new(direction.y, -direction.x)
}

fn line_positions(vertex: Vec2, normal: Vec2, width: f32) -> [Vec2; 2] {
    let p1 = normal * width + vertex;
    let p2 = normal * -width + vertex;
    [p1, p2]
}

fn line_mitter_positions(pos: &Vec2, dir: &Vec2, width: f32, prev_dir: Option<&Vec2>) -> [Vec2; 2] {
    // for math see
    // https://mattdesl.svbtle.com/drawing-lines-is-hard
    // https://cesium.com/blog/2013/04/22/robust-polyline-rendering-with-webgl/ "Vertex Shader Details"
    // https://www.npmjs.com/package/polyline-normals
    //
    let next_normal = get_normal(dir);

    if prev_dir.is_none() || dir == prev_dir.unwrap() {
        return line_positions(*pos, next_normal, width);
    }

    let prev_normal = get_normal(prev_dir.unwrap());
    let normal = (next_normal + prev_normal).normalize();
    let mitter_length = width / normal.dot(prev_normal);
    let mitter_length = mitter_length.min(width * 5.0);
    line_positions(*pos, normal, mitter_length)
}

impl Line {
    pub fn to_buffered_geometry_with(&self, opts: LineGeometryOpts) -> BufferedGeometry {
        let mut top_line = LineData::<f32>::new(self.default_width);
        let mut bottom_line = LineData::<f32>::new(self.default_width);
        let mut line_length = self.len_offset;

        for (prev, v, next) in self.iter().with_neighbours() {
            if prev.is_none() {
                top_line.add_width_data(v.pos, v.width, line_length);
                bottom_line.add_width_data(v.pos, v.width, line_length);
            }

            let new_points = line_mitter_positions(&v.pos, &v.dir, v.width, prev.map(|x| &x.dir));

            top_line.add_width_data(new_points[0], v.width, line_length);
            bottom_line.add_width_data(new_points[1], v.width, line_length);

            if next.is_none() {
                top_line.add_width_data(v.pos, v.width, line_length);
                bottom_line.add_width_data(v.pos, v.width, line_length);
            }

            line_length += v.len;
        }

        let mut buffer = vec![];
        let indices = vec![];

        let total_length = opts.total_length.unwrap_or(line_length);

        let n = usize::max(top_line.vert_count(), bottom_line.vert_count());

        for i in 0..n {
            let top = top_line.get(i);
            let bottom = bottom_line.get(i);

            let top_uv = Vec2::new(top.data / total_length, 0.0);
            let bottom_uv = Vec2::new(bottom.data / total_length, 1.0);

            let top_local_uv = Vec2::new(top.data / self.len, 0.0);
            let bottom_local_uv = Vec2::new(top.data / self.len, 1.0);

            // let top_vertex = VertexAttribute {
            //     position: top.pos,
            //     width: top.width,
            //     length: top.data,
            //     uv: top_uv,
            //     local_uv: top_local_uv,
            // };

            // let bottom_vertex = VertexAttribute {
            //     position: bottom.pos,
            //     width: bottom.width,
            //     length: bottom.data,
            //     uv: bottom_uv,
            //     local_uv: bottom_local_uv,
            // };

            // buffer.push(top_vertex);
            // buffer.push(bottom_vertex);
        }

        let indices_len = indices.len();

        let geom_layout = create_buffered_geometry_layout(VertexAttribute::vertex_layout());

        BufferedGeometry {
            buffer: Vec::from(bytemuck::cast_slice(&buffer)),
            rendering_primitive: RenderingPrimitive::TriangleStrip,
            indices: Some(indices),
            vertex_size: geom_layout.vertex_size,
            vertex_count: indices_len as u32,
            vertex_layout: geom_layout.vertex_layout,
        }
    }
}
