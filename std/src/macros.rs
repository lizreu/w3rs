#[macro_export]
macro_rules! log {
    ($l:expr, $($arg:tt)*) => {
        $crate::util::log($l, &format!($($arg)*));
    };

    (@to_level trace) => {
        $crate::util::LogLevel::Trace
    };

    (@to_level debug) => {
        $crate::util::LogLevel::Debug
    };

    (@to_level info) => {
        $crate::util::LogLevel::Info
    };

    (@to_level warn) => {
        $crate::util::LogLevel::Warn
    };

    (@to_level error) => {
        $crate::util::LogLevel::Error
    };

    (@to_level fatal) => {
        $crate::util::LogLevel::Fatal
    };

    (@to_level $l:ident) => {
        compile_error!(concat!("Unknown log level: ", stringify!($l)))
    };

    ($l:ident: $($arg:tt)*) => {
        $crate::util::log($crate::log!(@to_level $l), &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log!(trace: $($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log!(debug: $($arg)*)
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!(info: $($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!(warn: $($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!(error: $($arg)*)
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::log!(fatal: $($arg)*)
    };
}
