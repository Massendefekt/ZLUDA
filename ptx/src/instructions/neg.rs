use crate::types::scalar::ScalarType;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NegDetails {
    pub typ: ScalarType,
    pub flush_to_zero: Option<bool>,
}
