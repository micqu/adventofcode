use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    F32(f32),
    F64(f64),
    Str(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(x) => x.fmt(f),
            Self::I16(x) => x.fmt(f),
            Self::I32(x) => x.fmt(f),
            Self::I64(x) => x.fmt(f),
            Self::I128(x) => x.fmt(f),
            Self::Isize(x) => x.fmt(f),
            Self::U8(x) => x.fmt(f),
            Self::U16(x) => x.fmt(f),
            Self::U32(x) => x.fmt(f),
            Self::U64(x) => x.fmt(f),
            Self::U128(x) => x.fmt(f),
            Self::Usize(x) => x.fmt(f),
            Self::F32(x) => x.fmt(f),
            Self::F64(x) => x.fmt(f),
            Self::Str(x) => x.fmt(f),
        }
    }
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    }
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(i128, I128);
impl_from!(isize, Isize);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(u128, U128);
impl_from!(usize, Usize);
impl_from!(f32, F32);
impl_from!(f64, F64);
impl_from!(String, Str);

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Self::Str(sol.to_owned())
    }
}

pub trait IntoSolution<W> {
    fn solution(self) -> W;
}

macro_rules! into_solution {
    ($type_:ident) => {
        impl IntoSolution<Option<Solution>> for $type_ {
            fn solution(self) -> Option<Solution> {
                Some(self.into())
            }
        }
    }
}

into_solution!(i8);
into_solution!(i16);
into_solution!(i32);
into_solution!(i64);
into_solution!(i128);
into_solution!(isize);
into_solution!(u8);
into_solution!(u16);
into_solution!(u32);
into_solution!(u64);
into_solution!(u128);
into_solution!(usize);
into_solution!(f32);
into_solution!(f64);
into_solution!(String);

impl IntoSolution<Option<Solution>> for &str {
    fn solution(self) -> Option<Solution> {
        Some(self.into())
    }
}