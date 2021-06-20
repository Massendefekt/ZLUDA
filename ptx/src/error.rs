use std::{num::ParseIntError, num::ParseFloatError};

quick_error! {
    #[derive(Debug)]
    pub enum PtxError {
        ParseInt (err: ParseIntError) {
            from()
            display("{}", err)
            cause(err)
        }
        ParseFloat (err: ParseFloatError) {
            from()
            display("{}", err)
            cause(err)
        }
        SyntaxError {}
        NonF32Ftz {}
        WrongArrayType {}
        WrongVectorElement {}
        MultiArrayVariable {}
        ZeroDimensionArray {}
        ArrayInitalizer {}
        NonExternPointer {}
    }
}


