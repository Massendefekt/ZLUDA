use crate::types::Type;
use crate::types::pointer::PointerType;
use crate::types::scalar::LdStScalarType;

use std::convert::TryInto;

pub struct LdDetails {
    pub qualifier: LdStQualifier,
    pub state_space: LdStateSpace,
    pub caching: LdCacheOperator,
    pub typ: LdStType,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdCacheOperator {
    Cached,
    L2Only,
    Streaming,
    LastUse,
    Uncached,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdStQualifier {
    Weak,
    Volatile,
    Relaxed(MemScope),
    Acquire(MemScope),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MemScope {
    Cta,
    Gpu,
    Sys,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum LdStateSpace {
    Generic,
    Const,
    Global,
    Local,
    Param,
    Shared,
}

sub_type! {
    LdStType {
        Scalar(LdStScalarType),
        Vector(LdStScalarType, u8),
        // Used in generated code
        Pointer(PointerType, LdStateSpace),
    }
}
