use crate::{plugin::TagDescriptor, types::Signature};

pub struct Tag {
    pub sig: Signature,
    pub desc: TagDescriptor,
}
