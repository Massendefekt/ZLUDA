pub struct CvtaDetails {
    pub to: CvtaStateSpace,
    pub from: CvtaStateSpace,
    pub size: CvtaSize,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CvtaStateSpace {
    Generic,
    Const,
    Global,
    Local,
    Shared,
}

pub enum CvtaSize {
    U32,
    U64,
}
