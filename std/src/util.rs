#[repr(i32)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info  = 2,
    Warn  = 3,
    Error = 4,
    Fatal = 5,
}

pub fn log(level: LogLevel, msg: &str) {
    unsafe {
        extra_ffi::__log(level, msg.as_ptr(), msg.len());
    }
}

mod extra_ffi {
    extern "C" {
        pub fn __log(level: super::LogLevel, msg: *const u8, len: usize);
    }
}
