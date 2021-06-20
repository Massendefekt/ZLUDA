use crate::types::scalar::FloatType;
use crate::instructions::rcp::RoundingMode;

#[derive(Copy, Clone)]
pub struct SqrtDetails {
    pub typ: FloatType,
    pub flush_to_zero: Option<bool>,
    pub kind: SqrtKind,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SqrtKind {
    Approx,
    Rounding(RoundingMode),
}



