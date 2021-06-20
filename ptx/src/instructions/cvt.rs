use crate::error::PtxError;
use crate::instructions::rcp::RoundingMode;
use crate::types::scalar::{FloatType, IntType};

pub enum CvtDetails {
    IntFromInt(CvtIntToIntDesc),
    FloatFromFloat(CvtDesc<FloatType, FloatType>),
    IntFromFloat(CvtDesc<IntType, FloatType>),
    FloatFromInt(CvtDesc<FloatType, IntType>),
}

pub struct CvtIntToIntDesc {
    pub dst: IntType,
    pub src: IntType,
    pub saturate: bool,
}

pub struct CvtDesc<Dst, Src> {
    pub rounding: Option<RoundingMode>,
    pub flush_to_zero: Option<bool>,
    pub saturate: bool,
    pub dst: Dst,
    pub src: Src,
}

impl CvtDetails {
    pub fn new_int_from_int_checked(
        saturate: bool,
        dst: IntType,
        src: IntType,
        err: &mut Vec<PtxError>,
    ) -> Self {
        if saturate {
            if src.is_signed() {
                if dst.is_signed() && dst.width() >= src.width() {
                    err.push(PtxError::SyntaxError);
                }
            } else {
                if dst == src || dst.width() >= src.width() {
                    err.push(PtxError::SyntaxError);
                }
            }
        }
        CvtDetails::IntFromInt(CvtIntToIntDesc { dst, src, saturate })
    }

    pub fn new_float_from_int_checked(
        rounding: RoundingMode,
        flush_to_zero: bool,
        saturate: bool,
        dst: FloatType,
        src: IntType,
        err: &mut Vec<PtxError>,
    ) -> Self {
        if flush_to_zero && dst != FloatType::F32 {
            err.push(PtxError::NonF32Ftz);
        }
        CvtDetails::FloatFromInt(CvtDesc {
            dst,
            src,
            saturate,
            flush_to_zero: Some(flush_to_zero),
            rounding: Some(rounding),
        })
    }

    pub fn new_int_from_float_checked(
        rounding: RoundingMode,
        flush_to_zero: bool,
        saturate: bool,
        dst: IntType,
        src: FloatType,
        err: &mut Vec<PtxError>,
    ) -> Self {
        if flush_to_zero && src != FloatType::F32 {
            err.push(PtxError::NonF32Ftz);
        }
        CvtDetails::IntFromFloat(CvtDesc {
            dst,
            src,
            saturate,
            flush_to_zero: Some(flush_to_zero),
            rounding: Some(rounding),
        })
    }
}


