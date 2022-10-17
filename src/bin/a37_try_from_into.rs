use std::convert::TryFrom;
use std::num::ParseIntError;
use thiserror::Error;
/*
Topic: TryFrom/TryInto

Summary:
- A library is needed for an application to convert hex color codes into their component color values
(red, green and blue). Hex color codes consist of a hash symbol followed by six hex digits. Every two hex
digits represent a color component in the order of red, green, blue.

Example hex color codes:
#ffffff -> Rgb(255, 255, 255)
#001122 -> Rgb(0, 17, 34)

Requirements:
- create a program to convert a hex code (as &str) into an Rgb struct
- implement TryFrom to perform the conversion
- Utilize the question mark operator in your implement

Notes:
- see the 'from_str_radix' function in the stdlib docs for 'u8' to conver hex digits to 'u8'
- Hex digits use a radix value of 16
- utilize the 'thiserror' create for your error type
- Run 'cargo test --bin ' to test your implementation
 */
#[derive(Debug, Error)]
enum RgbError {
    #[error("hex colors must begin with a hash (#)")]
    MissingHash,
    #[error("failed to parse hex digit: {0}")]
    ParseError(std::num::ParseIntError),
    #[error("Invalid hex color length (must be 6")]
    LengthError,
}

#[derive(Debug, Eq, PartialEq)]
struct Rgb(u8, u8, u8);

impl TryFrom<&str> for Rgb {
    type Error = RgbError;
    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        if !hex.starts_with('#') {
            return Err(RgbError::MissingHash);
        }

        if hex.len() != 7 {
            return Err(RgbError::LengthError);
        }

        let (r, g, b) = (
            u8::from_str_radix(&hex[1..=2], 16)?,
            u8::from_str_radix(&hex[3..=4], 16)?,
            u8::from_str_radix(&hex[5..=6], 16)?,
        );
        Ok(Self(r, g, b))
    }
}

impl From<ParseIntError> for RgbError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseError(err)
    }
}

fn main() {
}

mod test {
    use super::Rgb;
    use std::convert::TryFrom;

    #[test]
    fn converts_valid_hex_color() {
        let expected = Rgb(255, 99, 71);
        let actual = Rgb::try_from("#ff6347");
        assert_eq!(
            actual.is_ok(),
            true,
            "valid hex code should be converted to Rgb"
        );

        assert_eq!(actual.unwrap(), expected, "wrong rgb value");
    }

    #[test]
    fn fails_on_invalid_hex_digits() {
        assert_eq!(
            Rgb::try_from("#0011yy").is_err(),
            true,
            "should be an error with invalid hex color"
        );
    }

    #[test]
    fn fails_when_missing_hash() {
        assert_eq!(
            Rgb::try_from("001100").is_err(),
            true,
            "should be an error when missing hash symbol"
        );
    }
}
