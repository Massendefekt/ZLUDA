use crate::instructions::arguments::*;
use crate::instructions::Instruction;
use crate::variable::MultiVariable;

pub struct PredAt<ID> {
    pub not: bool,
    pub label: ID,
}

pub enum Statement<P: ArgParams> {
    Label(P::Id),
    Variable(MultiVariable<P::Id>),
    Instruction(Option<PredAt<P::Id>>, Instruction<P>),
    Block(Vec<Statement<P>>),
}
