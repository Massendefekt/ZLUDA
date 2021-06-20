use crate::instructions::rcp::RoundingMode;
use crate::types::scalar::{UIntType, SIntType, FloatType};

#[derive(Copy, Clone)]
pub enum ArithDetails {
    Unsigned(UIntType),
    Signed(ArithSInt),
    Float(ArithFloat),
}

#[derive(Copy, Clone)]
pub struct ArithSInt {
    pub typ: SIntType,
    pub saturate: bool,
}

#[derive(Copy, Clone)]
pub struct ArithFloat {
    pub typ: FloatType,
    pub rounding: Option<RoundingMode>,
    pub flush_to_zero: Option<bool>,
    pub saturate: bool,
}
