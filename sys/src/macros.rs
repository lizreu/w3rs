#[macro_export]
macro_rules! define_handles {
    (
        $(
            $h:ident: ( $($parent:ident),* $(,)? )
        ),* $(,)?
    ) => {
        $(
            #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
            pub struct $h;

            impl $crate::base::JassHandle for $h {
                fn to_kind() -> $crate::base::JHandleKind {
                    $crate::base::JHandleKind::$h
                }
            }

            impl $crate::base::sealed::Sealed for $h {}

            impl $crate::base::SubtypeOf<$h> for $h {}

            $(
                impl $crate::base::SubtypeOf<$parent> for $h {}
            )*
        )*

        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum JHandleKind {
            j_handle,
            $(
                $h,
            )*
        }

        impl JHandleKind {
            pub fn to_str(&self) -> &'static str {
                match self {
                    JHandleKind::j_handle => "j_handle",
                    $(
                        JHandleKind::$h => stringify!($h),
                    )*
                }
            }

            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    "j_handle" => Some(JHandleKind::j_handle),
                    $(
                        stringify!(JHandleKind::$h) => Some(JHandleKind::$h),
                    )*
                    _ => None,
                }
            }

            pub fn is_supertype_of(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        (JHandleKind::$h, JHandleKind::$h) => true,
                        $(
                            (JHandleKind::$parent, JHandleKind::$h) => true,
                        )*
                    )*
                    _ => false,
                }
            }

            pub fn is_subtype_of(&self, other: &Self) -> bool {
                self == other || !self.is_supertype_of(other)
            }
        }
    };
}

#[macro_export]
macro_rules! declare_ffi {
    // (
    //     @decl $linkname:literal $rsname:ident($($arg:ident: $argty:ty),* $(,)? ) -> $ret:ty
    // ) => {
    //     #[link_name = $linkname]
    //     pub fn $rsname($($arg: $argty),*) -> $ret;
    // };

    // (
    //     @decl $linkname:literal $rsname:ident($($arg:ident: $argty:ty),* $(,)? )
    // ) => {
    //     #[link_name = $linkname]
    //     pub fn $rsname($($arg: $argty),*);
    // };

    // (
    //     $(
    //         <$linkname:literal> $([$attrib:ident])? $rsname:ident($($arg:ident: $argty:ty),* $(,)? ) $(-> $ret:ty)?
    //     );* $(;)?
    // ) => {
    //     extern "C" {
    //         $(
    //             $crate::declare_ffi!(@decl $linkname $rsname($($arg: $argty),*) $(-> $ret)?);
    //         )*
    //     }
    // };
    (
        $({
            link: $linkname:literal
            name: $rsname:ident
            raw: ($($rawarg:ident: $rawargty:ty),* $(,)? ) $(-> $rawret:ty)?;
            user: ($($urarg:ident: $urargty:ty),* $(,)? ) $(-> $urret:ty)? {
                $($body:tt)*
            };
        }),* $(,)?
    ) => {
        use std::ffi::{CStr, CString};
        use $crate::{Agent, Weak};
        use $crate::stringret;

        pub mod raw {
            use super::*;
            extern "C" {
                $(
                    #[link_name = $linkname]
                    pub fn $rsname($($rawarg: $rawargty),*) $(-> $rawret)?;
                )*
            }
        }

        $(
            #[inline(always)]
            pub fn $rsname($($urarg: $urargty),*) $(-> $urret)? {
                unsafe { $($body)* }
            }
        )*
    };
}
