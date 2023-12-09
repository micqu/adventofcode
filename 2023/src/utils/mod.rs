pub mod solution;

pub const ADJ_EIGHT: [(isize, isize); 8] = [ (-1, 1), (-1, 0), (-1, -1), (0, -1), (0, 1), (1, 1), (1, 0), (1, -1) ];
pub const ADJ_FOUR: [(isize, isize); 4] = [ (-1, 0), (0, -1), (0, 1), (1, 0) ];

pub fn length(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

pub fn parse_u32<T>(input: &mut T) -> Option<u32>
where
    T: Iterator<Item = char>,
{
    let mut value: Option<u32> = None;
    for char in input {
        if let Some(digit) = char.to_digit(10) {
            if let Some(current) = value {
                value = Some(current * 10 + digit);
            } else {
                value = Some(digit);
            }
        } else if value.is_some() {
            return value;
        }
    }

    value
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

parse_positive_number!(parse_u64, u64);
parse_positive_number!(parse_u128, u128);

macro_rules! parse_number {
    ($name:tt, $type:ident) => {
        pub fn $name<T>(input: &mut T) -> Option<$type>
        where
            T: Iterator<Item = char>,
        {
            let mut negative = false;
            let mut value: Option<$type> = None;
            for char in input {
                if let Some(digit) = char.to_digit(10) {
                    if let Some(current) = value {
                        value = Some(current * 10 + digit as $type);
                    } else {
                        value = Some(digit as $type);
                    }
                } else if char == '-' {
                    negative = true;
                } else if let Some(current) = value {
                    if negative {
                        return Some(-current);
                    }
                    return value;
                }
            }
        
            if negative {
                if let Some(current) = value {
                    return Some(-current);
                }
            }
            value
        }
    }
}

parse_number!(parse_i64, i64);