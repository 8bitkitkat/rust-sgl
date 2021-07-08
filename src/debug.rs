use std::ptr::null_mut;

pub type DebugCallbackFn = Box<dyn FnMut(DebugCallbackInfo, &str)>;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Source {
    Api = rgl::DEBUG_SOURCE_API,
    WindowSystem = rgl::DEBUG_SOURCE_WINDOW_SYSTEM,
    ShaderCompiler = rgl::DEBUG_SOURCE_SHADER_COMPILER,
    ThirdParty = rgl::DEBUG_SOURCE_THIRD_PARTY,
    Application = rgl::DEBUG_SOURCE_APPLICATION,
    Other = rgl::DEBUG_SOURCE_OTHER,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum MessageType {
    Error = rgl::DEBUG_TYPE_ERROR,
    DeprecatedBehavior = rgl::DEBUG_TYPE_DEPRECATED_BEHAVIOR,
    UndefinedBehavior = rgl::DEBUG_TYPE_UNDEFINED_BEHAVIOR,
    Portability = rgl::DEBUG_TYPE_PORTABILITY,
    Performance = rgl::DEBUG_TYPE_PERFORMANCE,
    Marker = rgl::DEBUG_TYPE_MARKER,
    PushGroup = rgl::DEBUG_TYPE_PUSH_GROUP,
    PopGroup = rgl::DEBUG_TYPE_POP_GROUP,
    Other = rgl::DEBUG_TYPE_OTHER,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Severity {
    Notification = rgl::DEBUG_SEVERITY_NOTIFICATION,
    Low = rgl::DEBUG_SEVERITY_LOW,
    Medium = rgl::DEBUG_SEVERITY_MEDIUM,
    High = rgl::DEBUG_SEVERITY_HIGH,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebugCallbackInfo {
    pub source: Source,
    pub message_type: MessageType,
    pub severity: Severity,
    pub id: u32,
}

static mut USER_CALLBACK: Option<DebugCallbackFn> = None;

extern "system" fn internal_debug_callback(
    source: u32,
    ty: u32,
    id: u32,
    severity: u32,
    _length: i32,
    message: *const i8,
    _: *mut std::os::raw::c_void,
) {
    if let Some(func) = unsafe { &mut USER_CALLBACK } {
        let (source, message_type, severity): (Source, MessageType, Severity) = unsafe {
            use std::mem::transmute;
            (transmute(source), transmute(ty), transmute(severity)) // todo bounds checking
        };

        let info = DebugCallbackInfo {
            source,
            message_type,
            severity,
            id,
        };

        let message = unsafe {
            String::from_utf8(std::ffi::CStr::from_ptr(message).to_bytes().to_vec()).unwrap()
        };

        func(info, &message);
    }
}

pub fn debug_message_callback(func: Option<DebugCallbackFn>) {
    unsafe {
        if func.is_some() {
            rgl::DebugMessageCallback(Some(internal_debug_callback), null_mut());
        } else {
            rgl::DebugMessageCallback(None, null_mut());
        }

        USER_CALLBACK = func;
    }
}
