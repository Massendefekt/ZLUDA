use crate::instructions::arguments::ArgParams;

use crate::instructions::ld::{LdStQualifier, LdStType};

pub struct StData {
    pub qualifier: LdStQualifier,
    pub state_space: StStateSpace,
    pub caching: StCacheOperator,
    pub typ: LdStType,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum StStateSpace {
    Generic,
    Global,
    Local,
    Param,
    Shared,
}

#[derive(PartialEq, Eq)]
pub enum StCacheOperator {
    Writeback,
    L2Only,
    Streaming,
    Writethrough,
}

pub struct Arg2St<P: ArgParams> {
    pub src1: P::Operand,
    pub src2: P::Operand,
}


