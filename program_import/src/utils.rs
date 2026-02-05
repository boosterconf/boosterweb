//! General utility functions

/// Serde deserialize bools from strings containing `"true"` or `"false"`
pub fn deserialize_bool_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: &str = serde::de::Deserialize::deserialize(deserializer)?;

    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(s, &["true", "false"])),
    }
}

/// English number ordinals
pub fn number_ordinal(num: u32) -> &'static str {
    match num % 100 {
        11..=13 => "th",
        x => match x % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

/// Make a String for displaying trees in the terminal. Takes a `Vec` of bools
/// where the bool is true if this is the last element.
pub fn tree_term(levels: Vec<bool>) -> String {
    let mut res = "".to_string();

    let mut iter = levels.into_iter().peekable();
    while let Some(is_last) = iter.next() {
        if iter.peek().is_some() {
            if is_last {
                res.push_str("    ");
            } else {
                res.push_str("│   ");
            }
        } else if is_last {
            res.push_str("└── ");
        } else {
            res.push_str("├── ");
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_ordinal() {
        assert_eq!("st".to_string(), number_ordinal(101));
        assert_eq!("nd".to_string(), number_ordinal(102));
        assert_eq!("rd".to_string(), number_ordinal(103));
        assert_eq!("st".to_string(), number_ordinal(1));
        assert_eq!("nd".to_string(), number_ordinal(2));
        assert_eq!("rd".to_string(), number_ordinal(3));
        assert_eq!("th".to_string(), number_ordinal(111));
        assert_eq!("th".to_string(), number_ordinal(112));
        assert_eq!("th".to_string(), number_ordinal(113));
        assert_eq!("th".to_string(), number_ordinal(115));
        assert_eq!("rd".to_string(), number_ordinal(123));
    }
}
