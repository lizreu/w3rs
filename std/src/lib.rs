extern crate alloc;

mod macros;

pub mod agentdata;
pub mod callbacks;
pub mod driver;
pub mod util;

pub mod object {
    pub mod player;
    pub mod timer;
    pub mod unit;
}

use std::panic::set_hook;

pub use w3_derive::cc;

#[no_mangle]
extern "C" fn init() {
    set_hook(Box::new(|info| {
        let backtrace = w3_sys::backtrace::read_backtrace();

        let location = info.location().unwrap();
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<dyn Any>",
            },
        };

        if !backtrace.is_empty() {
            let backtrace_msg = backtrace
                .iter()
                .map(|frame| {
                    let func_name = frame.func_name.to_str().unwrap();
                    let module_offset = frame.module_offset;
                    format!("{module_offset}: {func_name}")
                })
                .collect::<Vec<_>>()
                .join("\n");

            error!("panicked at '{msg}', {location}\n{backtrace_msg}");
        } else {
            error!("panicked at '{msg}', {location}");
        }
    }));
}

#[no_mangle]
extern "C" fn malloc(size: usize, align: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(size, align).unwrap();
    unsafe { std::alloc::alloc(layout) }
}

#[no_mangle]
extern "C" fn free(ptr: *mut u8, size: usize, align: usize) {
    let layout = std::alloc::Layout::from_size_align(size, align).unwrap();
    unsafe { std::alloc::dealloc(ptr, layout) }
}
