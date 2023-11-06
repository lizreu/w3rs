use core::marker::PhantomData;
use std::{
    fmt::{self, Formatter},
    num::{NonZeroI32, NonZeroU32},
};

pub use crate::gen::handles::*;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    value:   i32,
    _marker: PhantomData<fn() -> T>,
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Hash)]
pub struct Agent<T> {
    value:   NonZeroI32,
    _marker: PhantomData<fn() -> T>,
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Hash)]
pub struct Weak<T> {
    value:   NonZeroI32,
    _marker: PhantomData<fn() -> T>,
}

macro_rules! impl_display_debug_handle {
    ($n:ident, $t:ty) => {
        impl<T: JassHandle> std::fmt::Display for $t {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", stringify!($n))?;
                write!(f, "({}/{})", self.value, T::to_kind().to_str())
            }
        }

        impl<T: JassHandle> std::fmt::Debug for $t {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", stringify!($n))?;
                write!(f, "({}/{})", self.value, T::to_kind().to_str())
            }
        }
    };
}

impl_display_debug_handle!(Handle, Handle<T>);
impl_display_debug_handle!(Agent, Agent<T>);
impl_display_debug_handle!(Weak, Weak<T>);

impl<T: JassHandle> Handle<T> {
    pub fn invalid() -> Self {
        Self {
            value:   i32::MAX,
            _marker: PhantomData,
        }
    }

    pub fn upcast<O>(self) -> Handle<O>
    where
        O: JassHandle,
        T: SubtypeOf<O>,
    {
        Handle {
            value:   self.value,
            _marker: PhantomData,
        }
    }

    pub fn to_weak(self) -> Option<Weak<T>>
    where
        T: SubtypeOf<j_agent>,
    {
        Some(Weak {
            value:   NonZeroI32::new(self.value)?,
            _marker: PhantomData,
        })
    }

    pub fn value(self) -> i32
    where
        T: SubtypeOf<j_handle>,
    {
        self.value
    }
}

impl<T> Agent<T>
where
    T: JassHandle + SubtypeOf<j_agent>,
{
    pub fn to_handle(&self) -> Handle<T> {
        Handle {
            value:   self.value.get(),
            _marker: PhantomData,
        }
    }

    pub fn value(&self) -> i32 {
        self.value.get()
    }
}

impl<T> Drop for Agent<T> {
    fn drop(&mut self) {
        unsafe {
            __handle_refcnt_dec(self.value.get());
        }
    }
}

impl<T> Clone for Agent<T>
where
    T: JassHandle + SubtypeOf<j_agent>,
{
    fn clone(&self) -> Self {
        unsafe {
            __handle_refcnt_inc(self.value.get());
        }

        Self {
            value:   self.value,
            _marker: PhantomData,
        }
    }
}

impl<T> Weak<T>
where
    T: JassHandle + SubtypeOf<j_agent>,
{
    /// # Safety
    ///
    /// This must be called immediately after a `Weak<T>` has been acquired,
    /// or at least before control is returned to the engine. Otherwise,
    /// the handle id may be reused by the engine, and the returned `Agent<T>`
    /// might point to a different object.
    pub fn promote(self) -> Agent<T> {
        unsafe { __handle_refcnt_inc(self.value.get()) };

        Agent {
            value:   self.value,
            _marker: PhantomData,
        }
    }
}

pub trait JassHandle: sealed::Sealed {
    fn to_kind() -> JHandleKind;
}

pub trait SubtypeOf<Other>
where
    Other: JassHandle,
    Self: JassHandle + sealed::Sealed,
{
}

#[allow(non_camel_case_types)]
pub struct j_handle {}

impl JassHandle for j_handle {
    fn to_kind() -> JHandleKind {
        JHandleKind::j_handle
    }
}
impl sealed::Sealed for j_handle {}
impl SubtypeOf<j_handle> for j_handle {}

pub(crate) mod sealed {
    pub trait Sealed {}
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CodeId(pub NonZeroU32);

impl CodeId {
    pub const fn from_callback_id(value: u32) -> Self {
        Self(NonZeroU32::new(value + 1).unwrap())
    }

    pub const fn callback_id(self) -> u32 {
        self.0.get() - 1
    }

    pub const fn dead() -> Self {
        Self(NonZeroU32::new(u32::MAX).unwrap())
    }

    pub const fn is_dead(self) -> bool {
        self.0.get() == u32::MAX
    }
}

extern "C" {
    fn __handle_refcnt_inc(i: i32);
    fn __handle_refcnt_dec(i: i32);
    fn __handle_refcnt_get(i: i32) -> i32;
}
