use crate::error::PtxError;
use crate::types::*;
use crate::types::pointer::*;
use crate::types::scalar::*;

use std::convert::{TryFrom, TryInto};

#[derive(Clone)]
pub struct Variable<T, ID> {
    pub align: Option<u32>,
    pub v_type: T,
    pub name: ID,
    pub array_init: Vec<u8>,
}

#[derive(Eq, PartialEq, Clone)]
pub enum VariableType {
    Reg(VariableRegType),
    Local(VariableLocalType),
    Param(VariableParamType),
    Global(VariableGlobalType),
    Shared(VariableGlobalType),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StateSpace {
    Reg,
    Const,
    Global,
    Local,
    Shared,
    Param,
}

pub struct MultiVariable<ID> {
    pub var: Variable<VariableType, ID>,
    pub count: Option<u32>,
}

impl VariableType {
    pub fn to_type(&self) -> (StateSpace, Type) {
        match self {
            VariableType::Reg(t) => (StateSpace::Reg, t.clone().into()),
            VariableType::Local(t) => (StateSpace::Local, t.clone().into()),
            VariableType::Param(t) => (StateSpace::Param, t.clone().into()),
            VariableType::Global(t) => (StateSpace::Global, t.clone().into()),
            VariableType::Shared(t) => (StateSpace::Shared, t.clone().into()),
        }
    }
}

impl From<VariableType> for Type {
    fn from(t: VariableType) -> Self {
        match t {
            VariableType::Reg(t) => t.into(),
            VariableType::Local(t) => t.into(),
            VariableType::Param(t) => t.into(),
            VariableType::Global(t) => t.into(),
            VariableType::Shared(t) => t.into(),
        }
    }
}

sub_type! {
    VariableRegType {
        Scalar(ScalarType),
        Vector(SizedScalarType, u8),
        // Array type is used when emiting SSA statements at the start of a method
        Array(ScalarType, VecU32),
        // Pointer variant is used when passing around SLM pointer between
        // function calls for dynamic SLM
        Pointer(SizedScalarType, PointerStateSpace)
    }
}

sub_type! {
    VariableLocalType {
        Scalar(SizedScalarType),
        Vector(SizedScalarType, u8),
        Array(SizedScalarType, VecU32),
    }
}

impl TryFrom<VariableGlobalType> for VariableLocalType {
    type Error = PtxError;

    fn try_from(value: VariableGlobalType) -> Result<Self, Self::Error> {
        match value {
            VariableGlobalType::Scalar(t) => Ok(VariableLocalType::Scalar(t)),
            VariableGlobalType::Vector(t, len) => Ok(VariableLocalType::Vector(t, len)),
            VariableGlobalType::Array(t, len) => Ok(VariableLocalType::Array(t, len)),
            VariableGlobalType::Pointer(_, _) => Err(PtxError::ZeroDimensionArray),
        }
    }
}

sub_type! {
    VariableGlobalType {
        Scalar(SizedScalarType),
        Vector(SizedScalarType, u8),
        Array(SizedScalarType, VecU32),
        Pointer(SizedScalarType, PointerStateSpace),
    }
}

// For some weird reson this is illegal:
//   .param .f16x2 foobar;
// but this is legal:
//   .param .f16x2 foobar[1];
// even more interestingly this is legal, but only in .func (not in .entry):
//   .param .b32 foobar[]
sub_type! {
    VariableParamType {
        Scalar(LdStScalarType),
        Array(SizedScalarType, VecU32),
        Pointer(SizedScalarType, PointerStateSpace),
    }
}
