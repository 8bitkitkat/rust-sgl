use super::{Type, Usage};
use shrinkwraprs::Shrinkwrap;
use std::{os::raw::c_void, ptr::null};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Shrinkwrap)]
pub struct Buffer(u32);

impl Buffer {
    pub const NONE: Self = Self(0);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Shrinkwrap)]
pub struct VertexArray(u32);

impl VertexArray {
    pub const NONE: Self = Self(0);
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum BufferKind {
    Array = rgl::ARRAY_BUFFER,
    AtomicCounter = rgl::ATOMIC_COUNTER_BUFFER,
    CopyRead = rgl::COPY_READ_BUFFER,
    CopyWrite = rgl::COPY_WRITE_BUFFER,
    DispatchIndirect = rgl::DISPATCH_INDIRECT_BUFFER,
    DrawIndirect = rgl::DRAW_INDIRECT_BUFFER,
    ElementArray = rgl::ELEMENT_ARRAY_BUFFER,
    PixelPack = rgl::PIXEL_PACK_BUFFER,
}

pub fn gen_buffers<const N: usize>() -> [Buffer; N] {
    assert!(N < u32::MAX as usize);
    let mut buffers = [Buffer(0); N];
    unsafe { rgl::GenBuffers(N as i32, buffers.as_mut_ptr() as *mut u32) };
    buffers
}

pub fn gen_buffer() -> Buffer {
    gen_buffers::<1>()[0]
}

pub fn bind_buffer(target: BufferKind, buffer: Buffer) {
    unsafe { rgl::BindBuffer(target as u32, buffer.0) }
}

pub fn buffer_data<T>(target: BufferKind, data: &T, usage: Usage) {
    let size = std::mem::size_of_val(data) as isize;
    let data: *const std::os::raw::c_void = data as *const T as *const _;
    unsafe { rgl::BufferData(target as u32, size, data, usage as u32) }
}

pub unsafe fn buffer_data_ptr(target: BufferKind, size: isize, data: *const c_void, usage: Usage) {
    rgl::BufferData(target as u32, size, data, usage as u32)
}

pub fn gen_vertex_arrays<const N: usize>() -> [VertexArray; N] {
    assert!(N < u32::MAX as usize);
    let mut array = [VertexArray(0); N];
    unsafe { rgl::GenVertexArrays(N as i32, array.as_mut_ptr() as *mut u32) };
    array
}

pub fn gen_vertex_array() -> VertexArray {
    gen_vertex_arrays::<1>()[0]
}

pub fn bind_vertex_array(vertex_array: VertexArray) {
    unsafe { rgl::BindVertexArray(vertex_array.0) }
}

pub fn delete_buffers(buffers: &[Buffer]) {
    unsafe { rgl::DeleteBuffers(buffers.len() as i32 - 1, buffers.as_ptr() as *const u32) }
}

pub fn delete_buffer(buffer: Buffer) {
    delete_buffers(&[buffer])
}

pub fn delete_vertex_arrays(arrays: &[VertexArray]) {
    unsafe { rgl::DeleteVertexArrays(arrays.len() as i32 - 1, arrays.as_ptr() as *const u32) }
}

pub fn delete_vertex_array(array: VertexArray) {
    delete_vertex_arrays(&[array])
}

pub unsafe fn vertex_attrib_pointer_raw(
    index: u32,
    size: i32,
    ty: Type,
    normalized: bool,
    stride: i32,
    pointer: Option<*const c_void>,
) {
    rgl::VertexAttribPointer(
        index,
        size,
        ty as u32,
        normalized as u8,
        stride,
        if let Some(p) = pointer { p } else { null() },
    )
}

pub fn vertex_attrib_pointer(
    index: u32,
    size: i32,
    ty: Type,
    normalized: bool,
    stride: i32,
    offset: isize,
) {
    unsafe {
        rgl::VertexAttribPointer(
            index,
            size,
            ty as u32,
            normalized as u8,
            stride,
            offset as *const _,
        )
    }
}

pub unsafe fn vertex_attrib_ipointer(
    index: u32,
    size: i32,
    ty: Type,
    stride: i32,
    pointer: *const c_void,
) {
    debug_assert!(matches!(
        ty,
        Type::Byte
            | Type::UnsignedByte
            | Type::Short
            | Type::UnsignedShort
            | Type::Int
            | Type::UnsignedInt
    ));

    rgl::VertexAttribIPointer(index, size, ty as u32, stride, pointer)
}

// The type param is dropped because it only accepts `GL_DOUBLE` anyway
pub unsafe fn vertex_attrib_lpointer(index: u32, size: i32, stride: i32, pointer: *const c_void) {
    rgl::VertexAttribLPointer(index, size, rgl::DOUBLE, stride, pointer)
}

pub fn enable_vertex_attrib_array(index: u32) {
    unsafe { rgl::EnableVertexAttribArray(index) }
}
