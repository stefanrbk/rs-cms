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

pub type S15Fixed16Number = i32;
pub type U16Fixed16Number = u32;
pub type U1Fixed15Number = u16;
pub type U8Fixed8Number = u16;

pub type Result<T> = core::result::Result<T, &'static str>;

mod consts;
#[macro_use]
mod err;
mod functions;
pub mod io;
pub mod plugin;
pub mod state;
pub mod types;

pub use consts::*;
pub(crate) use err::*;
pub use functions::*;
