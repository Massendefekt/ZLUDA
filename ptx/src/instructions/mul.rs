use crate::instructions::add::ArithFloat;
use crate::types::scalar::{UIntType, SIntType};

#[derive(Copy, Clone)]
pub struct MulUInt {
    pub typ: UIntType,
    pub control: MulIntControl,
}

#[derive(Copy, Clone)]
pub struct MulSInt {
    pub typ: SIntType,
    pub control: MulIntControl,
}

#[derive(Copy, Clone)]
pub enum MulDetails {
    Unsigned(MulUInt),
    Signed(MulSInt),
    Float(ArithFloat),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MulIntControl {
    Low,
    High,
    Wide,
}
