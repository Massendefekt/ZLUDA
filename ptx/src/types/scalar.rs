sub_enum!(SizedScalarType {
    B8,
    B16,
    B32,
    B64,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F16,
    F16x2,
    F32,
    F64,
});

sub_enum!(LdStScalarType {
    B8,
    B16,
    B32,
    B64,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F16,
    F32,
    F64,
});

sub_enum!(SelpType {
    B16,
    B32,
    B64,
    U16,
    U32,
    U64,
    S16,
    S32,
    S64,
    F32,
    F64,
});


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ScalarType {
    B8,
    B16,
    B32,
    B64,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F16,
    F32,
    F64,
    F16x2,
    Pred,
}

sub_enum!(BitType { B8, B16, B32, B64 });

sub_enum!(IntType {
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64
});

sub_enum!(BooleanType {
    Pred,
    B16,
    B32,
    B64,
});

sub_enum!(UIntType { U8, U16, U32, U64 });

sub_enum!(SIntType { S8, S16, S32, S64 });

impl IntType {
    pub fn is_signed(self) -> bool {
        match self {
            IntType::U8 | IntType::U16 | IntType::U32 | IntType::U64 => false,
            IntType::S8 | IntType::S16 | IntType::S32 | IntType::S64 => true,
        }
    }

    pub fn width(self) -> u8 {
        match self {
            IntType::U8 => 1,
            IntType::U16 => 2,
            IntType::U32 => 4,
            IntType::U64 => 8,
            IntType::S8 => 1,
            IntType::S16 => 2,
            IntType::S32 => 4,
            IntType::S64 => 8,
        }
    }
}

sub_enum!(FloatType {
    F16,
    F16x2,
    F32,
    F64
});

impl ScalarType {
    pub fn size_of(self) -> u8 {
        match self {
            ScalarType::U8 => 1,
            ScalarType::S8 => 1,
            ScalarType::B8 => 1,
            ScalarType::U16 => 2,
            ScalarType::S16 => 2,
            ScalarType::B16 => 2,
            ScalarType::F16 => 2,
            ScalarType::U32 => 4,
            ScalarType::S32 => 4,
            ScalarType::B32 => 4,
            ScalarType::F32 => 4,
            ScalarType::U64 => 8,
            ScalarType::S64 => 8,
            ScalarType::B64 => 8,
            ScalarType::F64 => 8,
            ScalarType::F16x2 => 4,
            ScalarType::Pred => 1,
        }
    }
}

impl Default for ScalarType {
    fn default() -> Self {
        ScalarType::B8
    }
}

