pub mod solution;
pub mod vec2d;
pub mod vec3;

pub fn length(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

macro_rules! parse_positive_number {
    ($name:tt, $type:ident) => {
        pub fn $name<T>(input: &mut T) -> Option<$type>
        where
            T: Iterator<Item = char>,
        {
            let mut value: Option<$type> = None;
            for char in input {
                if let Some(digit) = char.to_digit(10) {
                    if let Some(current) = value {
                        value = Some(current * 10 + digit as $type);
                    } else {
                        value = Some(digit as $type);
                    }
                } else if value.is_some() {
                    return value;
                }
            }
        
            value
        }
    }
}

parse_positive_number!(parse_u32, u32);
parse_positive_number!(parse_u64, u64);
parse_positive_number!(parse_u128, u128);

// macro_rules! parse_number {
//     ($name:tt, $type:ident) => {
//         pub fn $name<T>(input: &mut T) -> Option<$type>
//         where
//             T: Iterator<Item = char>,
//         {
//             let mut negative = false;
//             let mut value: Option<$type> = None;
//             for char in input {
//                 if let Some(digit) = char.to_digit(10) {
//                     if let Some(current) = value {
//                         value = Some(current * 10 + digit as $type);
//                     } else {
//                         value = Some(digit as $type);
//                     }
//                 } else if char == '-' {
//                     negative = true;
//                 } else if let Some(current) = value {
//                     if negative {
//                         return Some(-current);
//                     }
//                     return value;
//                 }
//             }
        
//             if negative {
//                 if let Some(current) = value {
//                     return Some(-current);
//                 }
//             }
//             value
//         }
//     }
// }

// parse_number!(parse_i64, i64);

pub trait ToDigit {
    fn to_digit(&self) -> Option<u8>;
}

impl ToDigit for u8 {
    fn to_digit(&self) -> Option<u8> {
        if self.is_ascii_digit() {
            return Some(*self - b'0');
        }
        None
    }
}

pub trait Parsable<T>: Iterator {
    fn next_number(&mut self) -> Option<T>;
}

macro_rules! parsable_number {
    ($type:ident) => {
        impl<T: Iterator<Item = u8>> Parsable<$type> for T {
            fn next_number(&mut self) -> Option<$type> {
                let mut value: Option<$type> = None;
                for byte in self {
                    if let Some(digit) = byte.to_digit() {
                        if let Some(current) = value {
                            value = Some(current * 10 + digit as $type);
                        } else {
                            value = Some(digit as $type);
                        }
                    } else if value.is_some() {
                        return value;
                    }
                }

                value
            }
        }
    };
}

parsable_number!(u16);
parsable_number!(u32);
parsable_number!(u64);
parsable_number!(u128);
parsable_number!(usize);

macro_rules! parsable_negative_number {
    ($type:ident) => {
        impl<T: Iterator<Item = u8>> Parsable<$type> for T {
            fn next_number(&mut self) -> Option<$type> {
                let mut negative = false;
                let mut value: Option<$type> = None;
                for byte in self {
                    if let Some(digit) = byte.to_digit() {
                        if let Some(current) = value {
                            value = Some(current * 10 + digit as $type);
                        } else {
                            value = Some(digit as $type);
                        }
                    } else if let Some(value) = value {
                        if negative {
                            return Some(-value);
                        }
                        return Some(value);
                    } else if byte == b'-' {
                        negative = true;
                    } else {
                        negative = false;
                    }
                }

                if let Some(value) = value {
                    if negative {
                        return Some(-value);
                    }
                    return Some(value);
                }
                None
            }
        }
    };
}

parsable_negative_number!(i16);
parsable_negative_number!(i32);
parsable_negative_number!(i64);
parsable_negative_number!(i128);
parsable_negative_number!(isize);

macro_rules! parsable_float_number {
    ($type:ident) => {
        impl<T: Iterator<Item = u8>> Parsable<$type> for T {
            fn next_number(&mut self) -> Option<$type> {
                let mut negative = false;
                let mut value: Option<$type> = None;
                for byte in self {
                    if let Some(digit) = byte.to_digit() {
                        if let Some(current) = value {
                            value = Some(current * 10.0 + digit as $type);
                        } else {
                            value = Some(digit as $type);
                        }
                    } else if let Some(value) = value {
                        if negative {
                            return Some(-value);
                        }
                        return Some(value);
                    } else if byte == b'-' {
                        negative = true;
                    } else {
                        negative = false;
                    }
                }

                if let Some(value) = value {
                    if negative {
                        return Some(-value);
                    }
                    return Some(value);
                }
                None
            }
        }
    };
}

parsable_float_number!(f32);
parsable_float_number!(f64);

pub trait ToNumbers<T> {
    fn to_numbers(&self) -> Vec<T>;
}

macro_rules! to_numbers {
    ($type:ident) => {
        impl ToNumbers<$type> for &str
        {
            fn to_numbers(&self) -> Vec<$type> {
                let mut v = self.bytes();
                let mut ns = Vec::new();
                while let Some(n) = Parsable::<$type>::next_number(&mut v) {
                    ns.push(n);
                }
                ns
            }
        }
    };
}

to_numbers!(i16);
to_numbers!(isize);
to_numbers!(usize);

pub trait Transposable<T> {
    fn transpose(self) -> Vec<Vec<T>>;
}

impl<T> Transposable<T> for Vec<Vec<T>> {
    fn transpose(self) -> Vec<Vec<T>> {
        assert!(!self.is_empty());
        let len = self[0].len();
        let mut iters: Vec<_> = self.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }
}