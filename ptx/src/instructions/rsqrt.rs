use crate::types::scalar::FloatType;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RsqrtDetails {
    pub typ: FloatType,
    pub flush_to_zero: bool,
}

