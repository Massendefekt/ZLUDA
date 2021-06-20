use crate::instructions::arguments::ArgParams;

pub struct CallInst<P: ArgParams> {
    pub uniform: bool,
    pub ret_params: Vec<P::Id>,
    pub func: P::Id,
    pub param_list: Vec<P::Operand>,
}

