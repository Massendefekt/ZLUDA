use crate::types::Type;

#[derive(Clone)]
pub struct MovDetails {
    pub typ: Type,
    pub src_is_address: bool,
    // two fields below are in use by member moves
    pub dst_width: u8,
    pub src_width: u8,
    // This is in use by auto-generated movs
    pub relaxed_src2_conv: bool,
}

impl MovDetails {
    pub fn new(typ: Type) -> Self {
        MovDetails {
            typ,
            src_is_address: false,
            dst_width: 0,
            src_width: 0,
            relaxed_src2_conv: false,
        }
    }
}
