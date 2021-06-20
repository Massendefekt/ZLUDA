use std::{marker::PhantomData, };

#[derive(Copy, Clone)]
pub enum ImmediateValue {
    U64(u64),
    S64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Clone)]
pub enum Operand<Id> {
    Reg(Id),
    RegOffset(Id, i32),
    Imm(ImmediateValue),
    VecMember(Id, u8),
    VecPack(Vec<Id>),
}

pub trait ArgParams {
    type Id;
    type Operand;
}

pub struct ParsedArgParams<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> ArgParams for ParsedArgParams<'a> {
    type Id = &'a str;
    type Operand = Operand<&'a str>;
}

pub struct Arg1<P: ArgParams> {
    pub src: P::Id, // it is a jump destination, but in terms of operands it is a source operand
}

pub struct Arg1Bar<P: ArgParams> {
    pub src: P::Operand,
}

pub struct Arg2<P: ArgParams> {
    pub dst: P::Operand,
    pub src: P::Operand,
}

pub struct Arg2Ld<P: ArgParams> {
    pub dst: P::Operand,
    pub src: P::Operand,
}

pub struct Arg2Mov<P: ArgParams> {
    pub dst: P::Operand,
    pub src: P::Operand,
}

pub struct Arg3<P: ArgParams> {
    pub dst: P::Operand,
    pub src1: P::Operand,
    pub src2: P::Operand,
}

pub struct Arg4<P: ArgParams> {
    pub dst: P::Operand,
    pub src1: P::Operand,
    pub src2: P::Operand,
    pub src3: P::Operand,
}

pub struct Arg4Setp<P: ArgParams> {
    pub dst1: P::Id,
    pub dst2: Option<P::Id>,
    pub src1: P::Operand,
    pub src2: P::Operand,
}

pub struct Arg5Setp<P: ArgParams> {
    pub dst1: P::Id,
    pub dst2: Option<P::Id>,
    pub src1: P::Operand,
    pub src2: P::Operand,
    pub src3: P::Operand,
}
