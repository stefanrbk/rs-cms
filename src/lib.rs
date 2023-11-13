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

mod state;
mod types;
