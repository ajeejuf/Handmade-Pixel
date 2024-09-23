use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Username(String);

impl Username {
    pub fn parse(s: String) -> Result<Username, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', ' '];
        let contains_forbidden_characters = s.
            chars()
            .any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid username.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use crate::domain::Username;
    use claims::{ assert_err, assert_ok };

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(Username::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_grapheme_is_rejected() {
        let name = "ё".repeat(257);
        assert_err!(Username::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(Username::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(Username::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(Username::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "ajeej".to_string();
        assert_ok!(Username::parse(name));
    }
}