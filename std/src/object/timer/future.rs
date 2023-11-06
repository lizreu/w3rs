use std::{cell::Cell, future::Future, rc::Rc, task::Poll};

use futures::Stream;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum WaitState {
    Unscheduled,
    Scheduled,
    Done,
}

struct WaitInner {
    interval: std::time::Duration,
    state:    Cell<WaitState>,
}

struct IntervalInner {
    interval:      std::time::Duration,
    started:       Cell<bool>,
    steps_counted: Cell<u64>,
    steps_total:   Cell<u64>,
}

pub struct Wait {
    inner: Rc<WaitInner>,
}

pub struct Interval {
    inner: Rc<IntervalInner>,
}

/// Async version of [`super::once`].
pub fn wait(interval: std::time::Duration) -> Wait {
    Wait {
        inner: Rc::new(WaitInner {
            interval,
            state: Cell::new(WaitState::Unscheduled),
        }),
    }
}

/// Async version of [`super::repeating`].
pub fn interval(interval: std::time::Duration) -> Interval {
    Interval {
        inner: Rc::new(IntervalInner {
            interval,
            started: Cell::new(false),
            steps_counted: Cell::new(0),
            steps_total: Cell::new(0),
        }),
    }
}

impl Future for Wait {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let result = match self.inner.state.get() {
            WaitState::Unscheduled => {
                self.inner.state.set(WaitState::Scheduled);

                let waker = ctx.waker().clone();
                let inner = Rc::downgrade(&self.inner);
                super::once(self.inner.interval, move || {
                    if let Some(inner) = inner.upgrade() {
                        waker.wake();
                        inner.state.set(WaitState::Done);
                    }
                });

                Poll::Pending
            }
            WaitState::Scheduled => Poll::Pending,
            WaitState::Done => Poll::Ready(()),
        };

        result
    }
}

impl Stream for Interval {
    type Item = ();

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if !self.inner.started.get() {
            let waker = ctx.waker().clone();
            let inner = Rc::downgrade(&self.inner);
            super::repeating(self.inner.interval, move || {
                if let Some(inner) = inner.upgrade() {
                    waker.wake_by_ref();
                    inner.steps_total.set(inner.steps_total.get() + 1);
                    true
                } else {
                    false
                }
            });

            self.inner.started.set(true);

            Poll::Pending
        } else {
            let steps_counted = self.inner.steps_counted.get();
            let steps_total = self.inner.steps_total.get();

            if steps_counted < steps_total {
                self.inner.steps_counted.set(steps_counted + 1);
                Poll::Ready(Some(()))
            } else {
                Poll::Pending
            }
        }
    }
}
