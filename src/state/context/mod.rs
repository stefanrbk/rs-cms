use std::{sync::Arc, any::Any};

use crate::MAX_CHANNELS;

use super::ErrorHandlerLogFunction;

pub struct Context(Arc<ContextInner>);

struct ContextInner {
    alarm_codes: [u16; MAX_CHANNELS],
    adaptation_state: f64,
    user_data: Box<dyn Any + Sync + Send>,
    error_logger: Option<ErrorHandlerLogFunction>,
}
