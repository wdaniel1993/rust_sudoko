#[derive(Debug, PartialEq)]
pub struct Digit (u8);

impl Digit {
    pub fn new(digit: u8) -> Option<Digit> {
        if digit > 0 && digit <= 9 {
            Some(Digit(digit))
        } else {
            None
        }
    }
}

impl TryFrom<u8> for Digit {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match Digit::new(value) {
            Some(digit) => Ok(digit),
            None => Err("Digit only accepts value from 1 to 9!"),
        }
    }
}

impl From<Digit> for u8 {
    fn from(digit: Digit) -> Self {
        digit.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_creation_works() {
        let invalid_digit = Digit::new(11);
        let valid_digit = Digit::new(8);
        assert_eq!(invalid_digit, None);
        assert_eq!(valid_digit, Some(Digit(8)));
    }
}