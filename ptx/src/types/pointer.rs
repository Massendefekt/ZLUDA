use crate::instructions::ld::{LdStType, LdStateSpace};

use super::scalar::*;
use super::VecU32;

use std::convert::{From, TryFrom, TryInto};

sub_enum!(
    PointerStateSpace : LdStateSpace {
        Generic,
        Global,
        Const,
        Shared,
        Param,
    }
);

#[derive(PartialEq, Eq, Clone)]
pub enum PointerType {
    Scalar(ScalarType),
    Vector(ScalarType, u8),
    Array(ScalarType, VecU32),
    Pointer(ScalarType, LdStateSpace),
}

impl From<SizedScalarType> for PointerType {
    fn from(t: SizedScalarType) -> Self {
        PointerType::Scalar(t.into())
    }
}


impl TryFrom<PointerType> for SizedScalarType {
    type Error = ();

    fn try_from(value: PointerType) -> Result<Self, Self::Error> {
        match value {
            PointerType::Scalar(t) => Ok(t.try_into()?),
            PointerType::Vector(_, _) => Err(()),
            PointerType::Array(_, _) => Err(()),
            PointerType::Pointer(_, _) => Err(()),
        }
    }
}

impl From<LdStType> for PointerType {
    fn from(t: LdStType) -> Self {
        match t {
            LdStType::Scalar(t) => PointerType::Scalar(t.into()),
            LdStType::Vector(t, len) => PointerType::Vector(t.into(), len),
            LdStType::Pointer(PointerType::Scalar(scalar_type), space) => {
                PointerType::Pointer(scalar_type, space)
            }
            LdStType::Pointer(..) => unreachable!(),
        }
    }
}
