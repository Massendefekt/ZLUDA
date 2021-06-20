#[cfg(test)]
extern crate paste;
#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate quick_error;

extern crate bit_vec;
extern crate half;

#[macro_use]
extern crate bitflags;

lalrpop_mod!(
    #[allow(warnings)]
    ptx
);

#[macro_use]
mod macros;

pub mod ast;
mod error;
mod function;
mod instructions;
mod kernel;
mod statement;
mod types;
mod variable;
#[cfg(test)]
mod test;

pub use crate::ptx::ModuleParser;
pub use lalrpop_util::lexer::Token;
pub use lalrpop_util::ParseError;

use function::Function;
use instructions::arguments::{ArgParams, ParsedArgParams};
use statement::Statement;
use variable::{Variable, VariableType};

pub struct Module<'a> {
    pub version: (u8, u8),
    pub directives: Vec<Directive<'a, ParsedArgParams<'a>>>,
}

pub enum Directive<'a, P: ArgParams> {
    Variable(Variable<VariableType, P::Id>),
    Method(Function<'a, &'a str, Statement<P>>),
}


pub(crate) fn without_none<T>(x: Vec<Option<T>>) -> Vec<T> {
    x.into_iter().filter_map(|x| x).collect()
}

pub(crate) fn vector_index<'input>(
    inp: &'input str,
) -> Result<u8, ParseError<usize, lalrpop_util::lexer::Token<'input>, error::PtxError>> {
    match inp {
        "x" | "r" => Ok(0),
        "y" | "g" => Ok(1),
        "z" | "b" => Ok(2),
        "w" | "a" => Ok(3),
        _ => Err(ParseError::User {
            error: error::PtxError::WrongVectorElement,
        }),
    }
}
