#![deny(unsafe_code)]
#![cfg_attr(debug_assertions, allow(unused_macros))]
#![cfg_attr(debug_assertions, allow(unused_imports))]
#![cfg_attr(debug_assertions, allow(dead_code))]

/// Allow a block of 'unsafe' code with a reason.
///
/// The macro will expand to an 'unsafe' block.
macro_rules! unsafe_block {
    ($reason:tt => $body:expr) => {{
        #[allow(unsafe_code)]
        let r = unsafe { $body };
        r
    }};
}

macro_rules! err {
    (io => $t:ident, $s:tt) => {
        Err(std::io::Error::new(std::io::ErrorKind::$t, $s))
    };
    ($c:expr, $l:ident, $e:ident, $s:tt; io => $t:ident, $ss:tt) => {{
        $c.signal_error(
            log::Level::$l,
            crate::state::ErrorCode::$e,
            $s
        );
        Err(std::io::Error::new(std::io::ErrorKind::$t, $ss))}
    };
    ($c:expr, $l:ident, $e:ident, $s:tt; io => $error:expr) => { {
                $c.signal_error(
                log::Level::$l,
                crate::state::ErrorCode::$e,
                &$s
            );
            Err($error)
        }
    };
    ($c:expr, $l:ident, $e:ident, $s:tt, $($exprs:expr),*; io => $t:ident, $ss:tt) => { {
            $c.signal_error(
                log::Level::$l,
                crate::state::ErrorCode::$e,
                &format!($s, $($exprs),*)
            );
            Err(std::io::Error::new(std::io::ErrorKind::$t, $ss))
        }
    };
    ($c:expr, $l:ident, $e:ident, $s:tt, $($exprs:expr),*; io => $error:expr) => { {
                $c.signal_error(
                log::Level::$l,
                crate::state::ErrorCode::$e,
                &format!($s, $($exprs),*)
            );
            Err($error)
        }
    };
}

pub type S15Fixed16Number = i32;
pub type U16Fixed16Number = u32;
pub type U1Fixed15Number = u16;
pub type U8Fixed8Number = u16;

pub type Result<T> = core::result::Result<T, &'static str>;

mod consts;
pub mod io;
pub mod plugin;
pub mod state;
pub mod types;

pub use consts::*;
