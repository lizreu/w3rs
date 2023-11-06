#![allow(dead_code, unused_imports)]

use core::panic;
use std::num::NonZeroI32;

use slab::Slab;
use w3_sys::CodeId;

type CallbackFn = dyn FnMut() -> bool + 'static;

struct Callback {
    func: Box<CallbackFn>,
}

#[cfg(target_arch = "wasm32")]
static mut CURRENT_CALLBACK: CodeId = CodeId::dead();

#[cfg(target_arch = "wasm32")]
static mut DO_UNREGISTER: bool = false;

#[cfg(target_arch = "wasm32")]
static mut CALLBACKS: slab::Slab<Callback> = Slab::new();

pub fn register_callback<C>(_callback: C) -> CodeId
where
    C: FnMut() -> bool + 'static,
{
    #[cfg(target_arch = "wasm32")]
    unsafe {
        let id = CALLBACKS.insert(Callback {
            func: Box::new(_callback),
        });

        let id: u32 = id.try_into().expect("id is not u32");
        CodeId::from_callback_id(id)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        panic!("Code callbacks are only supported on wasm32 targets")
    }
}

pub fn register_callback_once<C>(_callback: C) -> CodeId
where
    C: FnOnce() + 'static,
{
    #[cfg(target_arch = "wasm32")]
    unsafe {
        let mut cb = Some(_callback);

        let id = CALLBACKS.insert(Callback {
            func: Box::new(move || {
                cb.take().expect("callback called more than once")();

                false
            }),
        });

        let id: u32 = id.try_into().expect("id is not u32");
        CodeId::from_callback_id(id)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        panic!("Code callbacks are only supported on wasm32 targets")
    }
}

pub fn replace_callback<C>(_id: CodeId, _callback: C)
where
    C: FnMut() -> bool + 'static,
{
    #[cfg(target_arch = "wasm32")]
    unsafe {
        let id = _id.callback_id().try_into().expect("id is not u32");

        if CALLBACKS.get(id).is_none() {
            panic!("attempted to replace callback that is not registered! {_id:?}");
        }

        CALLBACKS[id] = Callback {
            func: Box::new(_callback),
        };
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        panic!("Code callbacks are only supported on wasm32 targets")
    }
}

pub fn replace_callback_once<C>(_id: CodeId, _callback: C)
where
    C: FnOnce() + 'static,
{
    #[cfg(target_arch = "wasm32")]
    unsafe {
        let mut cb = Some(_callback);

        let id = _id.callback_id().try_into().expect("id is not u32");

        if CALLBACKS.get(id).is_none() {
            panic!("attempted to replace callback that is not registered! {_id:?}");
        }

        CALLBACKS[id] = Callback {
            func: Box::new(move || {
                cb.take().expect("callback called more than once")();

                false
            }),
        };
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        panic!("Code callbacks are only supported on wasm32 targets")
    }
}

pub fn unregister_callback(_id: CodeId) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        if CURRENT_CALLBACK == _id {
            DO_UNREGISTER = true;
        } else {
            let id = _id.callback_id().try_into().expect("id is not u32");

            if CALLBACKS.get(id).is_none() {
                panic!("attempted to unregister callback that is not registered! {_id:?}");
            }

            CALLBACKS.remove(id);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        panic!("Code callbacks are only supported on wasm32 targets")
    }
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
extern "C" fn native_callback(id: i32) {
    use crate::{info, warn};

    unsafe {
        let id: u32 = id.try_into().expect("id is not u32");

        if CALLBACKS.get((id - 1) as usize).is_none() {
            warn!("engine called callback {id}, but it is not registered");

            return;
        }

        CURRENT_CALLBACK = CodeId::from_callback_id(id);
        let keep = (CALLBACKS[(id - 1) as usize].func)();

        if !keep || DO_UNREGISTER {
            CALLBACKS.remove((id - 1) as usize);
            DO_UNREGISTER = false;
        }
    };
}
