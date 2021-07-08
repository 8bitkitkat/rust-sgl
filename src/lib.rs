//! Safe(er)/Simple openGL abstractions

pub extern crate gl as rgl;

use std::{
    ffi::c_void, ptr::null,
};
use enumflags2::{bitflags, BitFlags};

mod debug;
mod caps;
mod buffer;
mod program;
mod texture;

pub use rgl::load_with;
pub use debug::*;
pub use caps::*;
pub use buffer::*;
pub use program::*;
pub use texture::*;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Usage {
    StreamDraw = rgl::STREAM_DRAW,
    StreamRead = rgl::STREAM_READ,
    StreamCopy = rgl::STREAM_COPY,
    StaticDraw = rgl::STATIC_DRAW,
    StaticRead = rgl::STATIC_READ,
    StaticCopy = rgl::STATIC_COPY,
    DynamicDraw = rgl::DYNAMIC_DRAW,
    DynamicRead = rgl::DYNAMIC_READ,
    DynamicCopy = rgl::DYNAMIC_COPY,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Type {
    // These first few are accepted by both `vertex_attrib_pointer` and `vertex_attrib_ipointer`
    Byte = rgl::BYTE,                    // i8
    UnsignedByte = rgl::UNSIGNED_BYTE,   // u8
    Short = rgl::SHORT,                  // i16
    UnsignedShort = rgl::UNSIGNED_SHORT, // u16
    Int = rgl::INT,                      // i32
    UnsignedInt = rgl::UNSIGNED_INT,     // u32

    // These next few are accepted by `vertex_attrib_pointer` only
    HalfFloat = rgl::HALF_FLOAT, // f16
    Float = rgl::FLOAT,          // f32
    Double = rgl::DOUBLE,        // f64
    Fixed = rgl::FIXED,          // TODO: ?
    #[allow(non_camel_case_types)]
    Int_2_10_10_10_Rev = rgl::INT_2_10_10_10_REV, // TODO: ?
    #[allow(non_camel_case_types)]
    UnsignedInt_2_10_10_10_Rev = rgl::UNSIGNED_INT_2_10_10_10_REV, // TODO: ?
    #[allow(non_camel_case_types)]
    UnsignedInt_10F_11F_11F_Rev = rgl::UNSIGNED_INT_10F_11F_11F_REV, // TODO: ?
}

impl Type {
    // in bytes
    pub fn size(self) -> usize {
        use std::mem::size_of;
        match self {
            Type::Byte => size_of::<i8>(),
            Type::UnsignedByte => size_of::<u8>(),
            Type::Short => size_of::<i16>(),
            Type::UnsignedShort => size_of::<u16>(),
            Type::Int => size_of::<i32>(),
            Type::UnsignedInt => size_of::<u32>(),
            Type::HalfFloat => size_of::<f32>() / 2,
            Type::Float => size_of::<f32>(),
            Type::Double => size_of::<f64>(),
            // Type::Fixed => {}
            // Type::Int_2_10_10_10_Rev => {}
            // Type::UnsignedInt_2_10_10_10_Rev => {}
            // Type::UnsignedInt_10F_11F_11F_Rev => {}
            _ => todo!(),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum DrawMode {
    Points = rgl::POINTS,
    LineStrip = rgl::LINE_STRIP,
    LineLoop = rgl::LINE_LOOP,
    Lines = rgl::LINES,
    LineStripAdjacency = rgl::LINE_STRIP_ADJACENCY,
    LinesAdjacency = rgl::LINES_ADJACENCY,
    TriangleStrip = rgl::TRIANGLE_STRIP,
    TriangleFan = rgl::TRIANGLE_FAN,
    Triangles = rgl::TRIANGLES,
    TriangleStripAdjacency = rgl::TRIANGLE_STRIP_ADJACENCY,
    TriangleAdjacency = rgl::TRIANGLES_ADJACENCY,
    Patches = rgl::PATCHES,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum GlString {
    Vendor = rgl::VENDOR,
    Renderer = rgl::RENDERER,
    Version = rgl::VERSION,
    ShadingLanguageVersion = rgl::SHADING_LANGUAGE_VERSION,
}

impl GlString {
    pub fn get(&self) -> String {
        get_string(*self)
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum GlStringI {
    Extensions = rgl::EXTENSIONS,
}

impl GlStringI {
    pub fn get(&self, i: u32) -> String {
        get_string_i(*self, i)
    }
}

#[bitflags(default = Color | Depth | Stencil)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum BufferBit {
    Color = rgl::COLOR_BUFFER_BIT,
    Depth = rgl::DEPTH_BUFFER_BIT,
    Stencil = rgl::STENCIL_BUFFER_BIT,
}

impl BufferBit {
    #[inline]
    pub fn all() -> BitFlags<Self> {
        Self::Color | Self::Depth | Self::Stencil
    }
}

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe { rgl::ClearColor(r, g, b, a) }
}

pub fn clear_color_array(array: [f32; 4]) {
    clear_color(array[0], array[1], array[2], array[3])
}

pub fn clear(mask: BitFlags<BufferBit>) {
    unsafe { rgl::Clear(mask.bits()) }
}

pub fn draw_arrays(mode: DrawMode, first: i32, count: i32) {
    unsafe { rgl::DrawArrays(mode as u32, first, count) }
}

pub fn draw_elements(mode: DrawMode, count: i32, ty: Type, indices: Option<*const c_void>) {
    unsafe {
        rgl::DrawElements(
            mode as u32,
            count,
            ty as u32,
            if let Some(i) = indices { i } else { null() },
        )
    }
}

pub fn get_string(name: GlString) -> String {
    unsafe {
        let ptr = rgl::GetString(name as u32);
        String::from_utf8(
            std::ffi::CStr::from_ptr(ptr as *const i8)
                .to_bytes()
                .to_vec(),
        )
        .unwrap()
    }
}

pub fn get_string_i(name: GlStringI, i: u32) -> String {
    unsafe {
        let ptr = rgl::GetStringi(name as u32, i);
        String::from_utf8(
            std::ffi::CStr::from_ptr(ptr as *const i8)
                .to_bytes()
                .to_vec(),
        )
        .unwrap()
    }
}

pub fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { rgl::Viewport(x, y, width, height) }
}
