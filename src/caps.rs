#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Capability {
    Blend = rgl::BLEND,
    ColorLogicOp = rgl::COLOR_LOGIC_OP,
    CullFace = rgl::CULL_FACE,
    DebugOutput = rgl::DEBUG_OUTPUT,
    DebugOutputSynchronous = rgl::DEBUG_OUTPUT_SYNCHRONOUS,
    DepthClamp = rgl::DEPTH_CLAMP,
    DepthTest = rgl::DEPTH_TEST,
    Dither = rgl::DITHER,
    FramebufferSRBG = rgl::FRAMEBUFFER_SRGB,
    LineSmooth = rgl::LINE_SMOOTH,
    Multisample = rgl::MULTISAMPLE,
    PolygonOffsetFill = rgl::POLYGON_OFFSET_FILL,
    PolygonOffsetLine = rgl::POLYGON_OFFSET_LINE,
    PolygonOffsetPoint = rgl::POLYGON_OFFSET_POINT,
    PolygonSmooth = rgl::POLYGON_SMOOTH,
    PrimitiveRestart = rgl::PRIMITIVE_RESTART,
    PrimitiveRestartFixedIndex = rgl::PRIMITIVE_RESTART_FIXED_INDEX,
    RasterizerDiscard = rgl::RASTERIZER_DISCARD,
    SampleAlphaToCoverage = rgl::SAMPLE_ALPHA_TO_COVERAGE,
    SampleAlphaToOne = rgl::SAMPLE_ALPHA_TO_ONE,
    SampleCoverage = rgl::SAMPLE_COVERAGE,
    SampleShading = rgl::SAMPLE_SHADING,
    SampleMask = rgl::SAMPLE_MASK,
    ScissorTest = rgl::SCISSOR_TEST,
    StencilTest = rgl::STENCIL_TEST,
    TextureCubeMapSeamless = rgl::TEXTURE_CUBE_MAP_SEAMLESS,
    ProgramPointSize = rgl::PROGRAM_POINT_SIZE,

    ClipDistance0 = rgl::CLIP_DISTANCE0,
    ClipDistance1 = rgl::CLIP_DISTANCE1,
    ClipDistance2 = rgl::CLIP_DISTANCE2,
    ClipDistance3 = rgl::CLIP_DISTANCE3,
    ClipDistance4 = rgl::CLIP_DISTANCE4,
    ClipDistance5 = rgl::CLIP_DISTANCE5,
    ClipDistance6 = rgl::CLIP_DISTANCE6,
    ClipDistance7 = rgl::CLIP_DISTANCE7,
}

pub fn enable(cap: Capability) {
    unsafe { rgl::Enable(cap as u32) }
}

pub fn disable(cap: Capability) {
    unsafe { rgl::Disable(cap as u32) }
}

pub fn enable_i(cap: Capability, i: u32) {
    debug_assert!(matches!(cap, Capability::Blend | Capability::ScissorTest));
    unsafe { rgl::Enablei(cap as u32, i) }
}

pub fn disable_i(cap: Capability, i: u32) {
    debug_assert!(matches!(cap, Capability::Blend | Capability::ScissorTest));
    unsafe { rgl::Disablei(cap as u32, i) }
}
