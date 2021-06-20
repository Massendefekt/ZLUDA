use crate::types::scalar::{UIntType, SIntType, FloatType};
use crate::instructions::rcp::RoundingMode;

#[derive(Copy, Clone)]
pub enum DivDetails {
    Unsigned(UIntType),
    Signed(SIntType),
    Float(DivFloatDetails),
}

#[derive(Copy, Clone)]
pub struct DivFloatDetails {
    pub typ: FloatType,
    pub flush_to_zero: Option<bool>,
    pub kind: DivFloatKind,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DivFloatKind {
    Approx,
    Full,
    Rounding(RoundingMode),
}
