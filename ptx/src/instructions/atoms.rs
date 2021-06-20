use crate::types::scalar::{BitType, UIntType, SIntType, FloatType};
use crate::instructions::ld::MemScope;

#[derive(Copy, Clone)]
pub struct AtomDetails {
    pub semantics: AtomSemantics,
    pub scope: MemScope,
    pub space: AtomSpace,
    pub inner: AtomInnerDetails,
}

#[derive(Copy, Clone)]
pub enum AtomSemantics {
    Relaxed,
    Acquire,
    Release,
    AcquireRelease,
}

#[derive(Copy, Clone)]
pub enum AtomSpace {
    Generic,
    Global,
    Shared,
}

#[derive(Copy, Clone)]
pub enum AtomInnerDetails {
    Bit { op: AtomBitOp, typ: BitType },
    Unsigned { op: AtomUIntOp, typ: UIntType },
    Signed { op: AtomSIntOp, typ: SIntType },
    Float { op: AtomFloatOp, typ: FloatType },
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AtomBitOp {
    And,
    Or,
    Xor,
    Exchange,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AtomUIntOp {
    Add,
    Inc,
    Dec,
    Min,
    Max,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AtomSIntOp {
    Add,
    Min,
    Max,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AtomFloatOp {
    Add,
}

#[derive(Copy, Clone)]
pub struct AtomCasDetails {
    pub semantics: AtomSemantics,
    pub scope: MemScope,
    pub space: AtomSpace,
    pub typ: BitType,
}
