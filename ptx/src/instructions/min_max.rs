use crate::types::scalar::{SIntType, UIntType, FloatType};

#[derive(Copy, Clone)]
pub enum MinMaxDetails {
    Signed(SIntType),
    Unsigned(UIntType),
    Float(MinMaxFloat),
}

#[derive(Copy, Clone)]
pub struct MinMaxFloat {
    pub flush_to_zero: Option<bool>,
    pub nan: bool,
    pub typ: FloatType,
}
