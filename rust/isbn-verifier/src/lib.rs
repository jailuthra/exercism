/// Determines whether the supplied string is a valid ISBN number

use std::str::FromStr;

#[derive(Debug)]
struct ISBN {
    digits: [u32; 10],
}

impl FromStr for ISBN {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, &'static str> {
        let input = input.replace("-","");
        let mut isbn = ISBN {digits: [0; 10]};
        if input.len() != 10 {
            return Err("Invalid length");
        }
        for (i, c) in input.chars().enumerate() {
            match c.to_digit(10) {
                None => {
                    if c == 'X' && i == 9 {
                        isbn.digits[i] = 10;
                    } else {
                        return Err("Invalid digit")
                    }
                },
                Some(x) => {isbn.digits[i] = x; },
            };
        }
        return Ok(isbn);
    }
}

impl ISBN {
    fn validate(&self) -> bool {
        let sum = self.digits.into_iter().enumerate()
            .map(|(idx, val)| ((10 - idx) as u64) * (val as u64))
            .reduce(|p, val| p + val).expect("Invalid data, cannot compute product");
        return sum % 11 == 0;
    }
}

pub fn is_valid_isbn(isbn: &str) -> bool {
    match ISBN::from_str(isbn) {
        Ok(isbn) => return isbn.validate(),
        Err(s) => return false,
    }
}
