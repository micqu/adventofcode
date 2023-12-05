use std::str::Chars;

pub mod solution;

pub const ADJ_EIGHT: [(isize, isize); 8] = [ (-1, 1), (-1, 0), (-1, -1), (0, -1), (0, 1), (1, 1), (1, 0), (1, -1) ];
pub const ADJ_FOUR: [(isize, isize); 4] = [ (-1, 0), (0, -1), (0, 1), (1, 0) ];

pub fn parse_number(input: &mut Chars) -> Option<u32> {
    let mut value: Option<u32> = None;
    for char in input {
        if let Some(digit) = char.to_digit(10) {
            if let Some(current) = value {
                value = Some(current * 10 + digit);
            } else {
                value = Some(digit);
            }
        } else {
            return value;
        }
    }

    value
}