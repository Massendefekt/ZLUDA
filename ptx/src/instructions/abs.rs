use crate::types::scalar::ScalarType;

#[derive(Copy, Clone)]
pub struct AbsDetails {
    pub flush_to_zero: Option<bool>,
    pub typ: ScalarType,
}
