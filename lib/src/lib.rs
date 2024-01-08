#![deny(unsafe_code)]
#![cfg_attr(debug_assertions, allow(unused_macros))]
#![cfg_attr(debug_assertions, allow(unused_imports))]
#![cfg_attr(debug_assertions, allow(dead_code))]

/// Allow a block of 'unsafe' code with a reason.
///
/// The macro will expand to an 'unsafe' block.
///
/// # Example
/// ```rust
/// let zero: Option<i32> = Some(0);
/// return unsafe_block!("zero can only be `Some`", zero.unwrap_unchecked());
/// ```
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

pub type Sampler<T> = fn(r#in: &[T], out: &mut [T], cargo: &dyn Any) -> Result<()>;

#[macro_use]
mod err;

mod colorspace;
mod consts;
mod functions;
pub mod io;
pub mod plugin;
mod sem_ver;
pub mod sig;
pub mod state;
pub mod types;

use std::any::Any;

pub use consts::*;
pub(crate) use err::*;
pub use functions::*;
pub use sem_ver::SemVer;
