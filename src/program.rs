use shrinkwraprs::Shrinkwrap;
use std::{ffi::CString, ptr::null_mut};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ShaderKind {
    Compute = rgl::COMPUTE_SHADER,
    Vertex = rgl::VERTEX_SHADER,
    TessControl = rgl::TESS_CONTROL_SHADER,
    TessEvaluation = rgl::TESS_EVALUATION_SHADER,
    Geometry = rgl::GEOMETRY_SHADER,
    Fragment = rgl::FRAGMENT_SHADER,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ShaderProp {
    Type = rgl::SHADER_TYPE,
    DeleteStatus = rgl::DELETE_STATUS,
    CompileStatus = rgl::COMPILE_STATUS,
    InfoLogLength = rgl::INFO_LOG_LENGTH,
    SourceLength = rgl::SHADER_SOURCE_LENGTH,
}

// TODO: rest of the gets
impl ShaderProp {
    pub fn get_type(shader: Shader) -> ShaderKind {
        let mut kind: i32 = 0;
        unsafe {
            get_shader_iv(shader, Self::Type, &mut kind);
            std::mem::transmute(kind) // don't do this
        }
    }

    pub fn get_info_log_length(shader: Shader) -> i32 {
        let mut log_len: i32 = 0;
        unsafe { get_shader_iv(shader, Self::InfoLogLength, &mut log_len) };
        log_len
    }

    pub fn get_compile_status(shader: Shader) -> bool {
        let mut success: i32 = 0;
        unsafe { get_shader_iv(shader, Self::CompileStatus, &mut success) };
        success as u8 == rgl::TRUE
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ProgramProp {
    DeleteStatus = rgl::DELETE_STATUS,
    LinkStatus = rgl::LINK_STATUS,
    ValidateStatus = rgl::VALIDATE_STATUS,
    InfoLogLength = rgl::INFO_LOG_LENGTH,
    AttachedShaders = rgl::ATTACHED_SHADERS,
    ActiveAtomicCounterBuffers = rgl::ACTIVE_ATOMIC_COUNTER_BUFFERS,
    ActiveAttributes = rgl::ACTIVE_ATTRIBUTES,
    ActiveAttributeMaxLength = rgl::ACTIVE_ATTRIBUTE_MAX_LENGTH,
    ActiveUniforms = rgl::ACTIVE_UNIFORMS,
    ActiveUniformBlocks = rgl::ACTIVE_UNIFORM_BLOCKS,
    ActiveUniformBlockMaxNameLength = rgl::ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,
    ActiveUniformMaxLength = rgl::ACTIVE_UNIFORM_MAX_LENGTH,
    ComputeWorkGroupSize = rgl::COMPUTE_WORK_GROUP_SIZE,
    ProgramBinaryLength = rgl::PROGRAM_BINARY_LENGTH,
    TransformFeedbackBufferMode = rgl::TRANSFORM_FEEDBACK_BUFFER_MODE,
    TransformFeedbackVaryings = rgl::TRANSFORM_FEEDBACK_VARYINGS,
    TransformFeedbackVaryingMaxLength = rgl::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
    GeometryVerticesOut = rgl::GEOMETRY_VERTICES_OUT,
    GeometryInputType = rgl::GEOMETRY_INPUT_TYPE,
    GeometryOutputType = rgl::GEOMETRY_OUTPUT_TYPE,
}

// TODO: rest of the gets
impl ProgramProp {
    pub fn get_info_log_length(program: Program) -> i32 {
        let mut log_len: i32 = 0;
        unsafe { get_program_iv(program, Self::InfoLogLength, &mut log_len) };
        log_len
    }

    pub fn get_link_status(program: Program) -> bool {
        let mut success: i32 = 0;
        unsafe { get_program_iv(program, Self::LinkStatus, &mut success) };
        success as u8 == rgl::TRUE
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Shrinkwrap)]
pub struct Shader(u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Shrinkwrap)]
pub struct Program(u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Shrinkwrap)]
pub struct UniformLocation(i32);

pub fn create_shader(kind: ShaderKind) -> Shader {
    let int = unsafe { rgl::CreateShader(kind as u32) };
    Shader(int)
}

pub fn shader_source(shader: Shader, src: &str) {
    let str_ptr = [src.as_ptr()].as_ptr() as *const *const i8;
    let len_ptr = [src.len() as i32].as_ptr();
    unsafe { rgl::ShaderSource(shader.0, 1, str_ptr, len_ptr) };
}

/// Prefer [`ShaderProp`]'s get functions
pub unsafe fn get_shader_iv(shader: Shader, pname: ShaderProp, params: *mut i32) {
    rgl::GetShaderiv(shader.0, pname as u32, params);
}

/// Prefer [`get_shader_info_log`]
pub unsafe fn get_shader_info_log_raw(
    shader: Shader,
    buf_size: i32,
    length: *mut i32,
    info_log: *mut i8,
) {
    rgl::GetShaderInfoLog(shader.0, buf_size, length, info_log);
}

pub fn get_shader_info_log(shader: Shader) -> String {
    let mut log_len: i32 = 0;
    unsafe { get_shader_iv(shader, ShaderProp::InfoLogLength, &mut log_len) };

    if log_len == 0 {
        return "".to_string();
    }

    let log_len = log_len - 1;
    let mut vec = vec![0; log_len as usize];
    unsafe { get_shader_info_log_raw(shader, log_len, null_mut(), vec.as_mut_ptr() as *mut i8) };

    String::from_utf8(vec).unwrap()
}

pub fn delete_shader(shader: Shader) {
    unsafe { rgl::DeleteShader(shader.0) }
}

pub fn compile_shader(shader: Shader) {
    unsafe { rgl::CompileShader(shader.0) };
}

pub fn create_program() -> Program {
    let int = unsafe { rgl::CreateProgram() };
    Program(int)
}

pub fn attach_shader(program: Program, shader: Shader) {
    unsafe { rgl::AttachShader(program.0, shader.0) }
}

pub fn link_program(program: Program) {
    unsafe { rgl::LinkProgram(program.0) }
}

pub fn use_program(program: Program) {
    unsafe { rgl::UseProgram(program.0) }
}

pub fn delete_program(program: Program) {
    unsafe { rgl::DeleteProgram(program.0) }
}

/// Prefer [`ProgramProp`]'s get functions
pub unsafe fn get_program_iv(program: Program, pname: ProgramProp, params: *mut i32) {
    rgl::GetProgramiv(program.0, pname as u32, params);
}

/// Prefer [`get_shader_info_log`]
pub unsafe fn get_program_info_log_raw(
    program: Program,
    buf_size: i32,
    length: *mut i32,
    info_log: *mut i8,
) {
    rgl::GetProgramInfoLog(*program, buf_size, length, info_log);
}

pub fn get_program_info_log(program: Program) -> String {
    let mut log_len: i32 = 0;
    unsafe { get_program_iv(program, ProgramProp::InfoLogLength, &mut log_len) };

    if log_len == 0 {
        return "".to_string();
    }

    let log_len = log_len - 1;
    let mut vec = vec![0; log_len as usize];
    unsafe { get_program_info_log_raw(program, log_len, null_mut(), vec.as_mut_ptr() as *mut i8) };

    String::from_utf8(vec).unwrap()
}

pub fn get_uniform_location(program: Program, name: &str) -> UniformLocation {
    let name = CString::new(name).unwrap();
    let int = unsafe { gl::GetUniformLocation(*program, name.as_ptr()) };
    UniformLocation(int)
}

pub fn uniform_4f(location: UniformLocation, v0: f32, v1: f32, v2: f32, v3: f32) {
    unsafe { gl::Uniform4f(*location, v0, v1, v2, v3) }
}

pub fn uniform_4fs(location: UniformLocation, v: [f32; 4]) {
    uniform_4f(location, v[0], v[1], v[2], v[3])
}

pub fn uniform_1f(location: UniformLocation, v0: f32) {
    unsafe { gl::Uniform1f(*location, v0) }
}
