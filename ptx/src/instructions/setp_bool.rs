use crate::instructions::setp::SetpCompareOp;
use crate::types::scalar::ScalarType;

pub struct SetpBoolData {
    pub typ: ScalarType,
    pub flush_to_zero: Option<bool>,
    pub cmp_op: SetpCompareOp,
    pub bool_op: SetpBoolPostOp,
}

pub enum SetpBoolPostOp {
    And,
    Or,
    Xor,
}


