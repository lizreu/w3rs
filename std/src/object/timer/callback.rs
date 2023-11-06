use crate::callbacks::{register_callback, register_callback_once, unregister_callback};
use std::{cell::Cell, rc::Rc, time::Duration};
use w3_sys::{j_timer, native, Agent, CodeId};

/// Fully-featured API for WC3 timers.
///
/// Supports pausing and resuming timers, as well as querying the elapsed and remaining time.
///
/// Use this if you need to control the timer like an object, and not just to run a callback.
///
/// If you only need to run a callback, use [`once`] and [`repeating`] instead.
#[derive(Clone)]
pub struct Timer {
    inner: Rc<TimerInner>,
}

struct TimerInner {
    agent:   Agent<j_timer>,
    code_id: Cell<Option<CodeId>>,
}

impl Timer {
    pub fn once<C>(interval: Duration, callback: C) -> Self
    where
        C: FnOnce(&Self) + 'static,
    {
        let agent = native::create_timer()
            .expect("failed to create timer")
            .promote();

        let inner = Rc::new(TimerInner {
            agent,
            code_id: Cell::new(None),
        });

        let timer = Self { inner };

        let code_id = {
            let timer = timer.clone();

            register_callback_once(move || {
                callback(&timer);
                timer.destroy();
            })
        };

        timer.inner.code_id.set(Some(code_id));
        native::timer_start(&timer.inner.agent, interval.as_secs_f32(), false, code_id);

        timer
    }

    pub fn repeating<C>(interval: Duration, mut callback: C) -> Self
    where
        C: FnMut(&Self) + 'static,
    {
        let agent = native::create_timer()
            .expect("failed to create timer")
            .promote();

        let inner = Rc::new(TimerInner {
            agent,
            code_id: Cell::new(None),
        });

        let timer = Self { inner };

        let code_id = {
            let timer = timer.clone();

            register_callback(move || {
                callback(&timer);
                true
            })
        };

        timer.inner.code_id.set(Some(code_id));
        native::timer_start(&timer.inner.agent, interval.as_secs_f32(), true, code_id);

        timer
    }

    pub fn is_live(&self) -> bool {
        self.inner.code_id.get().is_some()
    }

    pub fn destroy(&self) {
        if let Some(code_id) = self.inner.code_id.take() {
            native::destroy_timer(&self.inner.agent);
            unregister_callback(code_id);
        };
    }

    pub fn elapsed(&self) -> Duration {
        if self.is_live() {
            let elapsed = native::timer_get_elapsed(&self.inner.agent);
            Duration::from_secs_f32(elapsed)
        } else {
            Duration::ZERO
        }
    }

    pub fn remaining(&self) -> Duration {
        if self.is_live() {
            let remaining = native::timer_get_remaining(&self.inner.agent);
            Duration::from_secs_f32(remaining)
        } else {
            Duration::ZERO
        }
    }

    pub fn pause(&self) {
        if self.is_live() {
            native::pause_timer(&self.inner.agent);
        }
    }

    pub fn resume(&self) {
        if self.is_live() {
            native::resume_timer(&self.inner.agent);
        }
    }
}

/// Light-weight one-shot alternative to [`Timer`] which offers no control over the timer.
///
/// Equivalent to [`Timer::once`], but without the ability to pause or resume the timer.
pub fn once<C>(interval: Duration, callback: C)
where
    C: FnOnce() + 'static,
{
    let agent = native::create_timer()
        .expect("failed to create timer")
        .promote();

    let code_id = {
        let agent = agent.clone();
        register_callback_once(move || {
            callback();
            native::destroy_timer(&agent);
        })
    };

    native::timer_start(&agent, interval.as_secs_f32(), false, code_id);
}

/// Light-weight repeating alternative to [`Timer`] which offers no control over the timer.
///
/// Equivalent to [`Timer::repeating`], but without the ability to pause or resume the timer.  
/// Return `false` from the callback to stop the timer.
pub fn repeating<C>(interval: Duration, mut callback: C)
where
    C: FnMut() -> bool + 'static,
{
    let agent = native::create_timer()
        .expect("failed to create timer")
        .promote();

    let code_id = {
        let agent = agent.clone();
        register_callback(move || {
            let do_repeat = callback();
            if !do_repeat {
                native::destroy_timer(&agent);
            }
            do_repeat
        })
    };

    native::timer_start(&agent, interval.as_secs_f32(), true, code_id);
}
