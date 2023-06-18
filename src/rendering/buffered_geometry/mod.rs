use std::collections::BTreeMap;

use bytemuck::cast_slice;
use serde::Serialize;
use serde_repr::Serialize_repr;

/// Sync with WebGL type values.
/// For possible values see: https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer
/// For numeric values see: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Constants
#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize_repr)]
pub enum AttributeType {
    Byte = 0x1400,
    UnsignedByte = 0x1401,
    Short = 0x1402,
    UnsignedShort = 0x1403,
    Float = 0x1406,
    HalfFloat = 0x140B,
}

/// For numeric values see: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Constants
#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize_repr)]
pub enum RenderingPrimitive {
    Points = 0x0000,
    Lines = 0x0001,
    LineLoop = 0x0002,
    LineStrip = 0x0003,
    Triangles = 0x0004,
    TriangleStrip = 0x0005,
    TriangleFan = 0x0006,
}

impl Default for RenderingPrimitive {
    fn default() -> Self {
        Self::Triangles
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum VertexFormat {
    /// Two unsigned bytes (u8). `uvec2` in shaders.
    Uint8x2 = 0,
    /// Four unsigned bytes (u8). `uvec4` in shaders.
    Uint8x4 = 1,
    /// Two signed bytes (i8). `ivec2` in shaders.
    Sint8x2 = 2,
    /// Four signed bytes (i8). `ivec4` in shaders.
    Sint8x4 = 3,
    /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2` in shaders.
    Unorm8x2 = 4,
    /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4` in shaders.
    Unorm8x4 = 5,
    /// Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2` in shaders.
    Snorm8x2 = 6,
    /// Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4` in shaders.
    Snorm8x4 = 7,
    /// Two unsigned shorts (u16). `uvec2` in shaders.
    Uint16x2 = 8,
    /// Four unsigned shorts (u16). `uvec4` in shaders.
    Uint16x4 = 9,
    /// Two signed shorts (i16). `ivec2` in shaders.
    Sint16x2 = 10,
    /// Four signed shorts (i16). `ivec4` in shaders.
    Sint16x4 = 11,
    /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2` in shaders.
    Unorm16x2 = 12,
    /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4` in shaders.
    Unorm16x4 = 13,
    /// Two signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec2` in shaders.
    Snorm16x2 = 14,
    /// Four signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec4` in shaders.
    Snorm16x4 = 15,
    /// Two half-precision floats (no Rust equiv). `vec2` in shaders.
    Float16x2 = 16,
    /// Four half-precision floats (no Rust equiv). `vec4` in shaders.
    Float16x4 = 17,
    /// One single-precision float (f32). `float` in shaders.
    Float32 = 18,
    /// Two single-precision floats (f32). `vec2` in shaders.
    Float32x2 = 19,
    /// Three single-precision floats (f32). `vec3` in shaders.
    Float32x3 = 20,
    /// Four single-precision floats (f32). `vec4` in shaders.
    Float32x4 = 21,
}

impl VertexFormat {
    /// Returns the byte size of the format.
    pub const fn byte_size(&self) -> u32 {
        match self {
            Self::Uint8x2 | Self::Sint8x2 | Self::Unorm8x2 | Self::Snorm8x2 => 2,
            Self::Uint8x4
            | Self::Sint8x4
            | Self::Unorm8x4
            | Self::Snorm8x4
            | Self::Uint16x2
            | Self::Sint16x2
            | Self::Unorm16x2
            | Self::Snorm16x2
            | Self::Float16x2
            | Self::Float32 => 4,
            Self::Uint16x4
            | Self::Sint16x4
            | Self::Unorm16x4
            | Self::Snorm16x4
            | Self::Float16x4
            | Self::Float32x2 => 8,
            Self::Float32x3 => 12,
            Self::Float32x4 => 16,
        }
    }

    pub const fn normalized(&self) -> bool {
        match self {
            Self::Unorm16x2
            | Self::Unorm16x4
            | Self::Unorm8x2
            | Self::Unorm8x4
            | Self::Snorm16x2
            | Self::Snorm16x4
            | Self::Snorm8x2
            | Self::Snorm8x4 => true,
            _ => false,
        }
    }

    pub const fn count(&self) -> u32 {
        match self {
            Self::Float32 => 1,
            Self::Uint8x2
            | Self::Sint8x2
            | Self::Unorm8x2
            | Self::Snorm8x2
            | Self::Uint16x2
            | Self::Sint16x2
            | Self::Unorm16x2
            | Self::Snorm16x2
            | Self::Float16x2
            | Self::Float32x2 => 2,
            Self::Float32x3 => 3,
            Self::Uint8x4
            | Self::Sint8x4
            | Self::Unorm8x4
            | Self::Snorm8x4
            | Self::Uint16x4
            | Self::Sint16x4
            | Self::Unorm16x4
            | Self::Snorm16x4
            | Self::Float16x4
            | Self::Float32x4 => 4,
        }
    }

    pub const fn attr_type(&self) -> AttributeType {
        match self {
            Self::Uint8x2 | Self::Unorm8x2 | Self::Uint8x4 | Self::Unorm8x4 => {
                AttributeType::UnsignedByte
            }
            Self::Sint8x2 | Self::Snorm8x2 | Self::Sint8x4 | Self::Snorm8x4 => AttributeType::Byte,
            Self::Uint16x2 | Self::Uint16x4 | Self::Unorm16x2 | Self::Unorm16x4 => {
                AttributeType::UnsignedShort
            }
            Self::Sint16x2 | Self::Sint16x4 | Self::Snorm16x2 | Self::Snorm16x4 => {
                AttributeType::Short
            }
            Self::Float16x2 | Self::Float16x4 => AttributeType::HalfFloat,
            Self::Float32x2 | Self::Float32x3 | Self::Float32x4 | Self::Float32 => {
                AttributeType::Float
            }
        }
    }

    pub const fn create_values(&self) -> VertexAttributeValues {
        match self {
            Self::Float32 => VertexAttributeValues::Float32(vec![]),
            Self::Uint8x2 => VertexAttributeValues::Uint8x4(vec![]),
            Self::Sint8x2 => VertexAttributeValues::Sint8x4(vec![]),
            Self::Unorm8x2 => VertexAttributeValues::Unorm8x4(vec![]),
            Self::Snorm8x2 => VertexAttributeValues::Snorm8x4(vec![]),
            Self::Uint16x2 => VertexAttributeValues::Uint16x4(vec![]),
            Self::Sint16x2 => VertexAttributeValues::Sint16x4(vec![]),
            Self::Unorm16x2 => VertexAttributeValues::Unorm16x4(vec![]),
            Self::Snorm16x2 => VertexAttributeValues::Snorm16x4(vec![]),
            Self::Float16x2 => VertexAttributeValues::Float32x4(vec![]),
            Self::Float32x2 => VertexAttributeValues::Float32x2(vec![]),
            Self::Float32x3 => VertexAttributeValues::Float32x3(vec![]),
            Self::Float32x4 => VertexAttributeValues::Float32x4(vec![]),
            Self::Uint8x4 => VertexAttributeValues::Uint8x4(vec![]),
            Self::Sint8x4 => VertexAttributeValues::Sint8x4(vec![]),
            Self::Unorm8x4 => VertexAttributeValues::Unorm8x4(vec![]),
            Self::Snorm8x4 => VertexAttributeValues::Snorm8x4(vec![]),
            Self::Uint16x4 => VertexAttributeValues::Uint16x4(vec![]),
            Self::Sint16x4 => VertexAttributeValues::Sint16x4(vec![]),
            Self::Unorm16x4 => VertexAttributeValues::Unorm16x4(vec![]),
            Self::Snorm16x4 => VertexAttributeValues::Snorm16x4(vec![]),
            Self::Float16x4 => VertexAttributeValues::Float32x4(vec![]),
        }
    }
}

#[derive(PartialEq)]
pub struct AttributeIndex {
    pub name: &'static str,
    pub format: VertexFormat,
    pub index: usize,
}

impl AttributeIndex {
    pub fn new(name: &'static str, format: VertexFormat, index: usize) -> AttributeIndex {
        AttributeIndex {
            name,
            format,
            index,
        }
    }
}

pub fn attr_idx(name: &'static str, format: VertexFormat, index: usize) -> AttributeIndex {
    AttributeIndex::new(name, format, index)
}

pub trait BufferedGeometryAttribute {
    fn attribute_index(&self) -> AttributeIndex;
    fn attribute_value(&self) -> VertexAttributeValue;
}

#[derive(Clone, Serialize, Debug)]
pub struct AttributeLayout {
    pub name: &'static str,
    pub size: u32,
    pub attr_type: AttributeType,
    pub normalized: bool,
    pub offset: u32,
}

#[derive(Clone, Serialize, Debug)]
pub struct BufferedGeometry {
    #[serde(with = "serde_bytes")]
    pub buffer: Vec<u8>,

    /// u32 indices converted to bytes, so they can be serialized efficiently.
    /// 4 bytes per index.
    #[serde(with = "serde_bytes")]
    pub indices: Option<Vec<u8>>,

    pub vertex_size: u32,
    pub vertex_count: u32,
    pub rendering_primitive: RenderingPrimitive,
    pub vertex_layout: Vec<AttributeLayout>,
}

pub trait ToBufferedGeometry {
    fn to_buffered_geometry_data(&self) -> BufferedGeometryData;
    fn to_buffered_geometry(&self) -> BufferedGeometry {
        self.to_buffered_geometry_data().to_buffered_geometry()
    }
}

pub struct VertexAttributeData {
    pub index: AttributeIndex,
    pub values: VertexAttributeValues,
}

struct BufferedGeometryData {
    attributes: BTreeMap<usize, VertexAttributeData>,
    indices: Option<Vec<u32>>,
    rendering_primitive: RenderingPrimitive,
}

impl BufferedGeometryData {
    pub fn new(rendering_primitive: RenderingPrimitive) -> BufferedGeometryData {
        BufferedGeometryData {
            attributes: BTreeMap::new(),
            indices: None,
            rendering_primitive,
        }
    }
}

impl BufferedGeometryData {
    fn init_attribute(&mut self, attr_idx: AttributeIndex) {
        self.attributes.insert(
            attr_idx.index,
            VertexAttributeData {
                index: attr_idx,
                values: attr_idx.format.create_values(),
            },
        );
    }

    fn to_buffered_geometry(&self) -> BufferedGeometry {
        let mut buffer = vec![];
        let mut vertex_layout = vec![];
        let mut vertex_size = 0;
        let mut vertex_count = 0;
        let mut has_values = true;

        for attr in self.attributes.values() {
            let format = attr.index.format;
            vertex_layout.push(AttributeLayout {
                name: attr.index.name,
                size: format.count(),
                attr_type: format.attr_type(),
                normalized: format.normalized(),
                offset: vertex_size,
            });
            vertex_size += format.byte_size();
        }

        while has_values {
            for attr in self.attributes.values() {
                let bytes = attr.values.get_bytes_at(vertex_count);
                if let Some(bytes) = bytes {
                    buffer.extend_from_slice(bytes);
                    if !has_values {
                        panic!("Vertex attribute values are not of the same length")
                    }
                } else {
                    has_values = false;
                }
            }
            vertex_count += 1;
        }

        let indices_len = match self.indices {
            Some(ref indices) => indices.len(),
            None => 0,
        };

        BufferedGeometry {
            buffer,
            rendering_primitive: RenderingPrimitive::Triangles,
            indices: self.indices.map(|v| cast_slice(&v).to_vec()),
            vertex_size,
            vertex_count: if indices_len > 0 {
                indices_len as u32
            } else {
                vertex_count as u32
            },
            vertex_layout,
        }
    }

    fn add_value(&mut self, idx: usize, value: VertexAttributeValue) {
        let vals = self.attributes.get_mut(&idx).unwrap();
        match &mut vals.values {
            VertexAttributeValues::Float32(v) => {
                if let VertexAttributeValue::Float32(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Sint32(v) => {
                if let VertexAttributeValue::Sint32(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Uint32(v) => {
                if let VertexAttributeValue::Uint32(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Float32x2(v) => {
                if let VertexAttributeValue::Float32x2(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Sint32x2(v) => {
                if let VertexAttributeValue::Sint32x2(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Uint32x2(v) => {
                if let VertexAttributeValue::Uint32x2(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Float32x3(v) => {
                if let VertexAttributeValue::Float32x3(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Sint32x3(v) => {
                if let VertexAttributeValue::Sint32x3(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Uint32x3(v) => {
                if let VertexAttributeValue::Uint32x3(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Float32x4(v) => {
                if let VertexAttributeValue::Float32x4(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Sint32x4(v) => {
                if let VertexAttributeValue::Sint32x4(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Uint32x4(v) => {
                if let VertexAttributeValue::Uint32x4(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Sint16x2(v) => {
                if let VertexAttributeValue::Sint16x2(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Snorm16x2(v) => {
                if let VertexAttributeValue::Snorm16x2(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Uint16x2(v) => {
                if let VertexAttributeValue::Uint16x2(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Unorm16x2(v) => {
                if let VertexAttributeValue::Unorm16x2(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Sint16x4(v) => {
                if let VertexAttributeValue::Sint16x4(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Snorm16x4(v) => {
                if let VertexAttributeValue::Snorm16x4(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Uint16x4(v) => {
                if let VertexAttributeValue::Uint16x4(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Unorm16x4(v) => {
                if let VertexAttributeValue::Unorm16x4(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Sint8x2(v) => {
                if let VertexAttributeValue::Sint8x2(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Snorm8x2(v) => {
                if let VertexAttributeValue::Snorm8x2(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Uint8x2(v) => {
                if let VertexAttributeValue::Uint8x2(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Unorm8x2(v) => {
                if let VertexAttributeValue::Unorm8x2(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Sint8x4(v) => {
                if let VertexAttributeValue::Sint8x4(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Snorm8x4(v) => {
                if let VertexAttributeValue::Snorm8x4(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Uint8x4(v) => {
                if let VertexAttributeValue::Uint8x4(val) = value {
                    v.push(val);
                }
            }
            VertexAttributeValues::Unorm8x4(v) => {
                if let VertexAttributeValue::Uint8x4(val) = value {
                    v.push(val);
                }
            }
        }
    }
}

pub enum VertexAttributeValues {
    Float32(Vec<f32>),
    Sint32(Vec<i32>),
    Uint32(Vec<u32>),
    Float32x2(Vec<[f32; 2]>),
    Sint32x2(Vec<[i32; 2]>),
    Uint32x2(Vec<[u32; 2]>),
    Float32x3(Vec<[f32; 3]>),
    Sint32x3(Vec<[i32; 3]>),
    Uint32x3(Vec<[u32; 3]>),
    Float32x4(Vec<[f32; 4]>),
    Sint32x4(Vec<[i32; 4]>),
    Uint32x4(Vec<[u32; 4]>),
    Sint16x2(Vec<[i16; 2]>),
    Snorm16x2(Vec<[i16; 2]>),
    Uint16x2(Vec<[u16; 2]>),
    Unorm16x2(Vec<[u16; 2]>),
    Sint16x4(Vec<[i16; 4]>),
    Snorm16x4(Vec<[i16; 4]>),
    Uint16x4(Vec<[u16; 4]>),
    Unorm16x4(Vec<[u16; 4]>),
    Sint8x2(Vec<[i8; 2]>),
    Snorm8x2(Vec<[i8; 2]>),
    Uint8x2(Vec<[u8; 2]>),
    Unorm8x2(Vec<[u8; 2]>),
    Sint8x4(Vec<[i8; 4]>),
    Snorm8x4(Vec<[i8; 4]>),
    Uint8x4(Vec<[u8; 4]>),
    Unorm8x4(Vec<[u8; 4]>),
}

impl VertexAttributeValues {
    fn get_bytes_at(&self, idx: usize) -> Option<&[u8]> {
        match self {
            VertexAttributeValues::Float32(v) => v.get(idx).map(|v| bytemuck::bytes_of(v)),
            VertexAttributeValues::Sint32(v) => v.get(idx).map(|v| bytemuck::bytes_of(v)),
            VertexAttributeValues::Uint32(v) => v.get(idx).map(|v| bytemuck::bytes_of(v)),
            VertexAttributeValues::Float32x2(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Sint32x2(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Uint32x2(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Float32x3(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Sint32x3(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Uint32x3(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Float32x4(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Sint32x4(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Uint32x4(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Sint16x2(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Snorm16x2(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Uint16x2(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Unorm16x2(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Sint16x4(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Snorm16x4(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Uint16x4(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Unorm16x4(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Sint8x2(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Snorm8x2(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Uint8x2(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Unorm8x2(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Sint8x4(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Snorm8x4(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Uint8x4(v) => v.get(idx).map(|v| cast_slice(v)),
            VertexAttributeValues::Unorm8x4(v) => v.get(idx).map(|v| cast_slice(v)),
        }
    }
}

pub enum VertexAttributeValue {
    Float32(f32),
    Sint32(i32),
    Uint32(u32),
    Float32x2([f32; 2]),
    Sint32x2([i32; 2]),
    Uint32x2([u32; 2]),
    Float32x3([f32; 3]),
    Sint32x3([i32; 3]),
    Uint32x3([u32; 3]),
    Float32x4([f32; 4]),
    Sint32x4([i32; 4]),
    Uint32x4([u32; 4]),
    Sint16x2([i16; 2]),
    Snorm16x2([i16; 2]),
    Uint16x2([u16; 2]),
    Unorm16x2([u16; 2]),
    Sint16x4([i16; 4]),
    Snorm16x4([i16; 4]),
    Uint16x4([u16; 4]),
    Unorm16x4([u16; 4]),
    Sint8x2([i8; 2]),
    Snorm8x2([i8; 2]),
    Uint8x2([u8; 2]),
    Unorm8x2([u8; 2]),
    Sint8x4([i8; 4]),
    Snorm8x4([i8; 4]),
    Uint8x4([u8; 4]),
    Unorm8x4([u8; 4]),
}
