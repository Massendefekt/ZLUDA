use crate::instructions::ld::LdStateSpace;

pub(crate) mod pointer;
pub(crate) mod scalar;

use pointer::PointerType;
use scalar::ScalarType;

pub type VecU32 = Vec<u32>;

#[derive(PartialEq, Eq, Clone)]
pub enum Type {
    Scalar(ScalarType),
    Vector(ScalarType, u8),
    Array(ScalarType, Vec<u32>),
    Pointer(PointerType, LdStateSpace),
}
