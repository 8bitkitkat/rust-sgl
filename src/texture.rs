#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TextureTarget {
    Texture1D = gl::TEXTURE_1D,
    Texture1DArray = gl::TEXTURE_1D_ARRAY,
    Texture2D = gl::TEXTURE_2D,
    Texture2DArray = gl::TEXTURE_2D_ARRAY,
    Texture2DMultiSample = gl::TEXTURE_2D_MULTISAMPLE,
    Texture2DMultiSampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
    Texture3D = gl::TEXTURE_3D,
    TextureCubeMap = gl::TEXTURE_CUBE_MAP,
    TextureCubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
    TextureRectangle = gl::TEXTURE_RECTANGLE,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TextureProp {
    DepthStencilTextureMode = gl::DEPTH_STENCIL_TEXTURE_MODE,
    TextureBaseLevel = gl::TEXTURE_BASE_LEVEL,
    TextureCompareFunc = gl::TEXTURE_COMPARE_FUNC,
    TextureCompareMode = gl::TEXTURE_COMPARE_MODE,
    TextureLodBias = gl::TEXTURE_LOD_BIAS,
    TextureMinFilter = gl::TEXTURE_MIN_FILTER,
    TextureMagFilter = gl::TEXTURE_MAG_FILTER,
    TextureMinLod = gl::TEXTURE_MIN_LOD,
    TextureMaxLod = gl::TEXTURE_MAX_LOD,
    TextureMaxLevel = gl::TEXTURE_MAX_LEVEL,
    TextureSwizzleR = gl::TEXTURE_SWIZZLE_R,
    TextureSwizzleG = gl::TEXTURE_SWIZZLE_G,
    TextureSwizzleB = gl::TEXTURE_SWIZZLE_B,
    TextureSwizzleA = gl::TEXTURE_SWIZZLE_A,
    TextureWrapS = gl::TEXTURE_WRAP_S,
    TextureWrapT = gl::TEXTURE_WRAP_T,
    TextureWrapR = gl::TEXTURE_WRAP_R,
    // TextureBorderColor = gl::TEXTURE_BORDER_COLOR,
    // TextureSwizzleRgba = gl::TEXTURE_SWIZZLE_RGBA,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TextureParam {
    MirroredRepeat = gl::MIRRORED_REPEAT,
}

pub fn tex_parameter_i(target: TextureTarget, prop: TextureProp, param: i32) {
    unsafe { gl::TexParameteri(target as u32, prop as u32, param) }
}
