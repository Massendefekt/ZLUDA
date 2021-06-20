use crate::types::scalar::{BitType, BooleanType, IntType, SelpType};

pub(crate) mod ld;
pub(crate) mod mov;
pub(crate) mod mul;
pub(crate) mod add;
pub(crate) mod setp;
pub(crate) mod setp_bool;
pub(crate) mod not;
pub(crate) mod bra;
pub(crate) mod cvt;
pub(crate) mod cvta;
pub(crate) mod shl;
pub(crate) mod shr;
pub(crate) mod st;
pub(crate) mod ret;
pub(crate) mod call;
pub(crate) mod abs;
pub(crate) mod mad;
pub(crate) mod or;
pub(crate) mod sub;
pub(crate) mod min_max;
pub(crate) mod rcp;
pub(crate) mod and;
pub(crate) mod selp;
pub(crate) mod bar;
pub(crate) mod div;
pub(crate) mod sqrt;
pub(crate) mod rsqrt;
pub(crate) mod neg;
pub(crate) mod atoms;

pub(crate) mod arguments;

use ld::LdDetails;
use mov::MovDetails;
use mul::MulDetails;
use add::ArithDetails;
use setp::SetpData;
use setp_bool::SetpBoolData;
use bra::BraData;
use cvt::CvtDetails;
use cvta::CvtaDetails;
use shl::ShlType;
use shr::ShrType;
use ret::RetData;
use st::{StData, Arg2St};
use call::CallInst;
use abs::AbsDetails;
use min_max::MinMaxDetails;
use rcp::RcpDetails;
use bar::BarDetails;
use div::DivDetails;
use sqrt::SqrtDetails;
use rsqrt::RsqrtDetails;
use neg::NegDetails;
use atoms::{AtomDetails, AtomCasDetails};
use arguments::*;

pub enum Instruction<P: ArgParams> {
    Ld(LdDetails, Arg2Ld<P>),
    Mov(MovDetails, Arg2Mov<P>),
    Mul(MulDetails, Arg3<P>),
    Add(ArithDetails, Arg3<P>),
    Setp(SetpData, Arg4Setp<P>),
    SetpBool(SetpBoolData, Arg5Setp<P>),
    Not(BooleanType, Arg2<P>),
    Bra(BraData, Arg1<P>),
    Cvt(CvtDetails, Arg2<P>),
    Cvta(CvtaDetails, Arg2<P>),
    Shl(ShlType, Arg3<P>),
    Shr(ShrType, Arg3<P>),
    St(StData, Arg2St<P>),
    Ret(RetData),
    Call(CallInst<P>),
    Abs(AbsDetails, Arg2<P>),
    Mad(MulDetails, Arg4<P>),
    Or(BooleanType, Arg3<P>),
    Sub(ArithDetails, Arg3<P>),
    Min(MinMaxDetails, Arg3<P>),
    Max(MinMaxDetails, Arg3<P>),
    Rcp(RcpDetails, Arg2<P>),
    And(BooleanType, Arg3<P>),
    Selp(SelpType, Arg4<P>),
    Bar(BarDetails, Arg1Bar<P>),
    Atom(AtomDetails, Arg3<P>),
    AtomCas(AtomCasDetails, Arg4<P>),
    Div(DivDetails, Arg3<P>),
    Sqrt(SqrtDetails, Arg2<P>),
    Rsqrt(RsqrtDetails, Arg2<P>),
    Neg(NegDetails, Arg2<P>),
    Sin { flush_to_zero: bool, arg: Arg2<P> },
    Cos { flush_to_zero: bool, arg: Arg2<P> },
    Lg2 { flush_to_zero: bool, arg: Arg2<P> },
    Ex2 { flush_to_zero: bool, arg: Arg2<P> },
    Clz { typ: BitType, arg: Arg2<P> },
    Brev { typ: BitType, arg: Arg2<P> },
    Popc { typ: BitType, arg: Arg2<P> },
    Xor { typ: BooleanType, arg: Arg3<P> },
    Bfe { typ: IntType, arg: Arg4<P> },
    Rem { typ: IntType, arg: Arg3<P> },
}


