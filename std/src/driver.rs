use std::{
    future::Future,
    pin::Pin,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

use slab::Slab;

static mut TASKS: Slab<Task> = Slab::new();

struct Task {
    future:   Option<Pin<Box<dyn Future<Output = ()> + 'static>>>,
    refcount: usize,
}

const VTABLE: RawWakerVTable =
    RawWakerVTable::new(waker_clone, waker_wake, waker_wake_by_ref, waker_drop);

#[inline(always)]
fn raw_waker(task_id: usize) -> RawWaker {
    RawWaker::new(task_id as *const (), &VTABLE)
}

#[inline(always)]
fn waker_clone(data: *const ()) -> RawWaker {
    let task_id = data as usize;

    unsafe {
        TASKS[task_id].refcount += 1;
    }

    RawWaker::new(task_id as *const (), &VTABLE)
}

#[inline(always)]
fn waker_wake(data: *const ()) {
    waker_wake_by_ref(data);
    waker_drop(data);
}

#[inline(always)]
fn waker_wake_by_ref(data: *const ()) {
    let task_id = data as usize;
    let task = unsafe { &mut TASKS[task_id] };

    match &mut task.future {
        Some(f) => {
            let f = f.as_mut();

            let waker = unsafe { Waker::from_raw(waker_clone(data)) };
            let mut cx = Context::from_waker(&waker);
            if f.poll(&mut cx).is_ready() {
                task.future = None;
            }
        }
        None => {
            panic!("Task {task_id} has not been stored yet!")
        }
    };
}

#[inline(always)]
fn waker_drop(data: *const ()) {
    let task_id = data as usize;
    let task = unsafe { &mut TASKS[task_id] };
    task.refcount -= 1;

    waker_check_refcount(data);
}

#[inline(always)]
fn waker_check_refcount(data: *const ()) {
    let task_id = data as usize;
    let refcount = unsafe { TASKS[task_id].refcount };
    if refcount == 0 {
        unsafe {
            TASKS.remove(task_id);
        }
    }
}

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    if !cfg!(target_arch = "wasm32") {
        panic!("Spawning futures is only supported on wasm32 targets")
    }

    let task_id = unsafe {
        TASKS.insert(Task {
            future:   None,
            refcount: 1,
        })
    };

    let mut future = Box::pin(future);

    let waker = unsafe { Waker::from_raw(raw_waker(task_id)) };
    if future
        .as_mut()
        .poll(&mut Context::from_waker(&waker))
        .is_pending()
    {
        unsafe {
            TASKS[task_id].future = Some(Box::pin(future));
        }
    }
}
