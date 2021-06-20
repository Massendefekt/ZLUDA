use crate::instructions::ld::LdStateSpace;
use crate::types::{Type, pointer::PointerType, scalar::ScalarType};
use crate::variable::*;

pub type KernelArgument<ID> = Variable<KernelArgumentType, ID>;

#[derive(PartialEq, Eq, Clone)]
pub enum KernelArgumentType {
    Normal(VariableParamType),
    Shared,
}

impl From<KernelArgumentType> for Type {
    fn from(this: KernelArgumentType) -> Self {
        match this {
            KernelArgumentType::Normal(typ) => typ.into(),
            KernelArgumentType::Shared => {
                Type::Pointer(PointerType::Scalar(ScalarType::B8), LdStateSpace::Shared)
            }
        }
    }
}

