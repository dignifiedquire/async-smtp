//! Utils for string manipulation

use std::fmt::{Display, Formatter, Result as FmtResult};

/// Encode a string as xtext
///
/// xtext is defined in <https://www.rfc-editor.org/rfc/rfc3461>
#[derive(Debug)]
pub struct XText<'a>(pub &'a str);

impl Display for XText<'_> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for c in self.0.chars() {
            if c < '!' || c == '+' || c == '=' {
                write!(f, "+{:X}", c as u8)?;
            } else {
                write!(f, "{c}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::XText;

    #[test]
    fn test() {
        for (input, expect) in [
            ("bjorn", "bjorn"),
            ("bjørn", "bjørn"),
            ("Ø+= ❤️‰", "Ø+2B+3D+20❤️‰"),
            ("+", "+2B"),
        ] {
            assert_eq!(format!("{}", XText(input)), expect.to_string());
        }
    }
}
