#[derive(Copy, Clone)]
pub struct RcpDetails {
    pub rounding: Option<RoundingMode>,
    pub flush_to_zero: Option<bool>,
    pub is_f64: bool,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum RoundingMode {
    NearestEven,
    Zero,
    NegativeInf,
    PositiveInf,
}
