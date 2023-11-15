use crate::{
    state::Context,
    types::{Pipeline, Profile},
    Result,
};

pub type IntentFn = fn(
    context_id: &Context,
    n_profiles: usize,
    intents: Box<[u32]>,
    profiles: Box<[Profile]>,
    bpc: Box<[bool]>,
    adaptation_states: Box<[f64]>,
    flags: u32,
) -> Result<Pipeline>;
