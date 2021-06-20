use crate::instructions::arguments::ParsedArgParams;
use crate::instructions::ld::LdStateSpace;
use crate::kernel::KernelArgument;
use crate::statement::Statement;
use crate::types::Type;
use crate::types::{scalar::*, pointer::*};
use crate::variable::*;

pub enum MethodDecl<'a, ID> {
    Func(Vec<FnArgument<ID>>, ID, Vec<FnArgument<ID>>),
    Kernel {
        name: &'a str,
        in_args: Vec<KernelArgument<ID>>,
    },
}

pub struct Function<'a, ID, S> {
    pub func_directive: MethodDecl<'a, ID>,
    pub body: Option<Vec<S>>,
}

pub type ParsedFunction<'a> = Function<'a, &'a str, Statement<ParsedArgParams<'a>>>;

pub type FnArgument<ID> = Variable<FnArgumentType, ID>;

#[derive(PartialEq, Eq, Clone)]
pub enum FnArgumentType {
    Reg(VariableRegType),
    Param(VariableParamType),
    Shared,
}

impl FnArgumentType {
    pub fn to_type(&self, is_kernel: bool) -> Type {
        if is_kernel {
            self.to_kernel_type()
        } else {
            self.to_func_type()
        }
    }

    pub fn to_kernel_type(&self) -> Type {
        match self {
            FnArgumentType::Reg(x) => x.clone().into(),
            FnArgumentType::Param(x) => x.clone().into(),
            FnArgumentType::Shared => {
                Type::Pointer(PointerType::Scalar(ScalarType::B8), LdStateSpace::Shared)
            }
        }
    }

    pub fn to_func_type(&self) -> Type {
        match self {
            FnArgumentType::Reg(x) => x.clone().into(),
            FnArgumentType::Param(VariableParamType::Scalar(t)) => {
                Type::Pointer(PointerType::Scalar((*t).into()), LdStateSpace::Param)
            }
            FnArgumentType::Param(VariableParamType::Array(t, dims)) => Type::Pointer(
                PointerType::Array((*t).into(), dims.clone()),
                LdStateSpace::Param,
            ),
            FnArgumentType::Param(VariableParamType::Pointer(t, space)) => Type::Pointer(
                PointerType::Pointer((*t).into(), (*space).into()),
                LdStateSpace::Param,
            ),
            FnArgumentType::Shared => {
                Type::Pointer(PointerType::Scalar(ScalarType::B8), LdStateSpace::Shared)
            }
        }
    }

    pub fn is_param(&self) -> bool {
        match self {
            FnArgumentType::Param(_) => true,
            _ => false,
        }
    }
}
