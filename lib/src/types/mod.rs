#[inline(always)]
fn dswap<T>(x: &mut T, y: &mut T)
where
    T: Copy,
{
    (*x, *y) = (*y, *x)
}

mod curve;
mod date_time;
mod format;
mod interp_params;
mod lab;
mod lch;
mod mat;
mod pipeline;
mod position;
mod profile;
mod response;
mod signature;
mod stage;
mod transform;
mod vec;
mod xyy;
mod xyz;

pub use date_time::DateTimeNumber;
pub use format::{pixel_type, Format};
pub use interp_params::{InterpFn, InterpFunction, InterpParams};
pub use lab::{Lab, LabEncoded};
pub use lch::LCh;
pub use mat::MAT3;
pub use pipeline::Pipeline;
pub use position::PositionNumber;
pub use profile::Profile;
pub use response::ResponseNumber;
pub use signature::Signature;
pub use stage::{Stage, StageDupFn, StageEvalFn};
pub use transform::*;
pub use vec::VEC3;
pub use xyy::XYY;
pub use xyz::{XYZEncoded, XYZNumber, XYZ};
