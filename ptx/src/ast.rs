use crate::error::PtxError;
use crate::types::scalar::{ScalarType, SizedScalarType};

use half::f16;
use std::{convert::From, mem, str::FromStr};

pub trait UnwrapWithVec<E, To> {
    fn unwrap_with(self, errs: &mut Vec<E>) -> To;
}

impl<R: Default, EFrom: std::convert::Into<EInto>, EInto> UnwrapWithVec<EInto, R>
    for Result<R, EFrom>
{
    fn unwrap_with(self, errs: &mut Vec<EInto>) -> R {
        self.unwrap_or_else(|e| {
            errs.push(e.into());
            R::default()
        })
    }
}

impl<
        R1: Default,
        EFrom1: std::convert::Into<EInto>,
        R2: Default,
        EFrom2: std::convert::Into<EInto>,
        EInto,
    > UnwrapWithVec<EInto, (R1, R2)> for (Result<R1, EFrom1>, Result<R2, EFrom2>)
{
    fn unwrap_with(self, errs: &mut Vec<EInto>) -> (R1, R2) {
        let (x, y) = self;
        let r1 = x.unwrap_with(errs);
        let r2 = y.unwrap_with(errs);
        (r1, r2)
    }
}

pub enum VectorPrefix {
    V2,
    V4,
}

pub enum NumsOrArrays<'a> {
    Nums(Vec<(&'a str, u32)>),
    Arrays(Vec<NumsOrArrays<'a>>),
}

impl<'a> NumsOrArrays<'a> {
    pub fn to_vec(self, typ: SizedScalarType, dimensions: &mut [u32]) -> Result<Vec<u8>, PtxError> {
        self.normalize_dimensions(dimensions)?;
        let sizeof_t = ScalarType::from(typ).size_of() as usize;
        let result_size = dimensions.iter().fold(sizeof_t, |x, y| x * (*y as usize));
        let mut result = vec![0; result_size];
        self.parse_and_copy(typ, sizeof_t, dimensions, &mut result)?;
        Ok(result)
    }

    fn normalize_dimensions(&self, dimensions: &mut [u32]) -> Result<(), PtxError> {
        match dimensions.first_mut() {
            Some(first) => {
                if *first == 0 {
                    *first = match self {
                        NumsOrArrays::Nums(v) => v.len() as u32,
                        NumsOrArrays::Arrays(v) => v.len() as u32,
                    };
                }
            }
            None => return Err(PtxError::ZeroDimensionArray),
        }
        for dim in dimensions {
            if *dim == 0 {
                return Err(PtxError::ZeroDimensionArray);
            }
        }
        Ok(())
    }

    fn parse_and_copy(
        &self,
        t: SizedScalarType,
        size_of_t: usize,
        dimensions: &[u32],
        result: &mut [u8],
    ) -> Result<(), PtxError> {
        match dimensions {
            [] => unreachable!(),
            [dim] => match self {
                NumsOrArrays::Nums(vec) => {
                    if vec.len() > *dim as usize {
                        return Err(PtxError::ZeroDimensionArray);
                    }
                    for (idx, (val, radix)) in vec.iter().enumerate() {
                        Self::parse_and_copy_single(t, idx, val, *radix, result)?;
                    }
                }
                NumsOrArrays::Arrays(_) => return Err(PtxError::ZeroDimensionArray),
            },
            [first_dim, rest @ ..] => match self {
                NumsOrArrays::Arrays(vec) => {
                    if vec.len() > *first_dim as usize {
                        return Err(PtxError::ZeroDimensionArray);
                    }
                    let size_of_element = rest.iter().fold(size_of_t, |x, y| x * (*y as usize));
                    for (idx, this) in vec.iter().enumerate() {
                        this.parse_and_copy(
                            t,
                            size_of_t,
                            rest,
                            &mut result[(size_of_element * idx)..],
                        )?;
                    }
                }
                NumsOrArrays::Nums(_) => return Err(PtxError::ZeroDimensionArray),
            },
        }
        Ok(())
    }

    fn parse_and_copy_single(
        t: SizedScalarType,
        idx: usize,
        str_val: &str,
        radix: u32,
        output: &mut [u8],
    ) -> Result<(), PtxError> {
        match t {
            SizedScalarType::B8 | SizedScalarType::U8 => {
                Self::parse_and_copy_single_t::<u8>(idx, str_val, radix, output)?;
            }
            SizedScalarType::B16 | SizedScalarType::U16 => {
                Self::parse_and_copy_single_t::<u16>(idx, str_val, radix, output)?;
            }
            SizedScalarType::B32 | SizedScalarType::U32 => {
                Self::parse_and_copy_single_t::<u32>(idx, str_val, radix, output)?;
            }
            SizedScalarType::B64 | SizedScalarType::U64 => {
                Self::parse_and_copy_single_t::<u64>(idx, str_val, radix, output)?;
            }
            SizedScalarType::S8 => {
                Self::parse_and_copy_single_t::<i8>(idx, str_val, radix, output)?;
            }
            SizedScalarType::S16 => {
                Self::parse_and_copy_single_t::<i16>(idx, str_val, radix, output)?;
            }
            SizedScalarType::S32 => {
                Self::parse_and_copy_single_t::<i32>(idx, str_val, radix, output)?;
            }
            SizedScalarType::S64 => {
                Self::parse_and_copy_single_t::<i64>(idx, str_val, radix, output)?;
            }
            SizedScalarType::F16 => {
                Self::parse_and_copy_single_t::<f16>(idx, str_val, radix, output)?;
            }
            SizedScalarType::F16x2 => todo!(),
            SizedScalarType::F32 => {
                Self::parse_and_copy_single_t::<f32>(idx, str_val, radix, output)?;
            }
            SizedScalarType::F64 => {
                Self::parse_and_copy_single_t::<f64>(idx, str_val, radix, output)?;
            }
        }
        Ok(())
    }

    fn parse_and_copy_single_t<T: Copy + FromStr>(
        idx: usize,
        str_val: &str,
        _radix: u32, // TODO: use this to properly support hex literals
        output: &mut [u8],
    ) -> Result<(), PtxError>
    where
        T::Err: Into<PtxError>,
    {
        let typed_output = unsafe {
            std::slice::from_raw_parts_mut::<T>(
                output.as_mut_ptr() as *mut _,
                output.len() / mem::size_of::<T>(),
            )
        };
        typed_output[idx] = str_val.parse::<T>().map_err(|e| e.into())?;
        Ok(())
    }
}

pub enum ArrayOrPointer {
    Array { dimensions: Vec<u32>, init: Vec<u8> },
    Pointer,
}

bitflags! {
    pub struct LinkingDirective: u8 {
        const NONE = 0b000;
        const EXTERN = 0b001;
        const VISIBLE = 0b10;
        const WEAK = 0b100;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_fails_multiple_0_dmiensions() {
        let inp = NumsOrArrays::Nums(Vec::new());
        assert!(inp.to_vec(SizedScalarType::B8, &mut vec![0, 0]).is_err());
    }

    #[test]
    fn array_fails_on_empty() {
        let inp = NumsOrArrays::Nums(Vec::new());
        assert!(inp.to_vec(SizedScalarType::B8, &mut vec![0]).is_err());
    }

    #[test]
    fn array_auto_sizes_0_dimension() {
        let inp = NumsOrArrays::Arrays(vec![
            NumsOrArrays::Nums(vec![("1", 10), ("2", 10)]),
            NumsOrArrays::Nums(vec![("3", 10), ("4", 10)]),
        ]);
        let mut dimensions = vec![0u32, 2];
        assert_eq!(
            vec![1u8, 2, 3, 4],
            inp.to_vec(SizedScalarType::B8, &mut dimensions).unwrap()
        );
        assert_eq!(dimensions, vec![2u32, 2]);
    }

    #[test]
    fn array_fails_wrong_structure() {
        let inp = NumsOrArrays::Arrays(vec![
            NumsOrArrays::Nums(vec![("1", 10), ("2", 10)]),
            NumsOrArrays::Arrays(vec![NumsOrArrays::Nums(vec![("1", 10)])]),
        ]);
        let mut dimensions = vec![0u32, 2];
        assert!(inp.to_vec(SizedScalarType::B8, &mut dimensions).is_err());
    }

    #[test]
    fn array_fails_too_long_component() {
        let inp = NumsOrArrays::Arrays(vec![
            NumsOrArrays::Nums(vec![("1", 10), ("2", 10), ("3", 10)]),
            NumsOrArrays::Nums(vec![("4", 10), ("5", 10)]),
        ]);
        let mut dimensions = vec![0u32, 2];
        assert!(inp.to_vec(SizedScalarType::B8, &mut dimensions).is_err());
    }
}
