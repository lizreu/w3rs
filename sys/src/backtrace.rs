use std::ffi::CString;

#[repr(C)]
struct FrameRaw {
    func_name:     *const i8,
    module_offset: u32,
}

pub struct Frame {
    pub func_name:     CString,
    pub module_offset: u32,
}

pub fn read_backtrace() -> Vec<Frame> {
    let raw = unsafe { read_backtrace_inner() };

    match raw {
        Some(raw) => raw
            .iter()
            .map(|frame| Frame {
                func_name:     unsafe { CString::from_raw(frame.func_name as *mut _) },
                module_offset: frame.module_offset,
            })
            .collect(),
        None => Vec::new(),
    }
}

#[cfg(target_arch = "wasm32")]
unsafe fn read_backtrace_inner() -> Option<Box<[FrameRaw]>> {
    let data = __backtrace();

    if data == 0 {
        return None;
    }

    let data: u64 = std::mem::transmute(data);
    let ptr = (data & 0xFFFFFFFF) as usize;
    let len = (data >> 32) as usize;

    let wide_ptr: *mut [FrameRaw] = std::ptr::slice_from_raw_parts_mut(ptr as *mut FrameRaw, len);
    Some(Box::from_raw(wide_ptr))
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn read_backtrace_inner() -> Option<Box<[FrameRaw]>> {
    unimplemented!("Backtraces are only supported on wasm32 targets")
}

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn __backtrace() -> i64;
}
