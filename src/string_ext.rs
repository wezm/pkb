pub trait StringExt {
    fn humanise(&self) -> String;

    /// Return a 'slugified' version of the string suitable for use in a URL path.
    ///
    /// "This: is an example! 😬" → "this-is-an-example-grimacing"
    fn to_slug(&self) -> String;
}

impl<T> StringExt for T
where
    T: AsRef<str>,
{
    fn humanise(&self) -> String {
        // When on newer compiler:
        // self.as_ref().replace(['_', '.'], " ")
        let pattern: &[char] = &['_', '.'];
        self.as_ref().replace(pattern, " ")
    }

    fn to_slug(&self) -> String {
        if self.as_ref().is_empty() {
            return String::new();
        }

        let as_ascii = deunicode::deunicode_with_tofu_cow(self.as_ref(), "-");
        let mut slug = String::with_capacity(as_ascii.len());
        for mut byte in as_ascii.bytes() {
            // special handling for some symbols
            match byte {
                // skip apostrophe so that contractions aren't separated by a hyphen
                b'\'' => continue,
                b'@' => {
                    add_word(&mut slug, "at");
                    continue;
                }
                b'&' => {
                    add_word(&mut slug, "and");
                    continue;
                }
                b'/' => {
                    add_word(&mut slug, "slash");
                    continue;
                }
                b'+' => {
                    add_word(&mut slug, "plus");
                    continue;
                }
                _ => {}
            }

            if !byte.is_ascii_alphanumeric() {
                byte = b'-'
            }

            if byte == b'-' {
                // skip leading hyphens and consecutive hyphens
                if matches!(slug.as_bytes().last(), Some(b'-') | None) {
                    continue;
                }
            }

            // Safety: Cast is safe as String will remain valid UTF-8 as deunicode guarantees the
            // result will be ASCII.
            slug.push(byte.to_ascii_lowercase() as char);
        }

        // By now the slug should have no leading hyphens or consecutive hyphens but it may have a
        // trailing hyphen, which we want to trim.
        if slug.ends_with('-') {
            slug.truncate(slug.len() - 1);
        }

        slug
    }
}

fn add_word(slug: &mut String, word: &str) {
    match slug.as_bytes().last() {
        Some(b'-') | None => {}
        Some(_) => slug.push('-'),
    }
    slug.push_str(word);
    slug.push('-');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_slug() {
        assert_eq!("Æneid".to_slug(), "aeneid");
        assert_eq!("étude".to_slug(), "etude");
        assert_eq!("北亰".to_slug(), "bei-jing");
        assert_eq!("ᔕᓇᓇ".to_slug(), "shanana");
        assert_eq!("げんまい茶".to_slug(), "genmaicha");
        assert_eq!("🦄☣".to_slug(), "unicorn-biohazard");
        assert_eq!("👨‍👩‍👧‍👦".to_slug(), "man-woman-girl-boy");
        assert_eq!("👩‍🚀".to_slug(), "woman-rocket");
        assert_eq!("  one \t\ntwo  ".to_slug(), "one-two");
        assert_eq!("--one- -two--".to_slug(), "one-two");
        assert_eq!("control: \x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x7F".to_slug(), "control");
        assert_eq!(
            "punctuation: !@#$%^&*()_+-=[]{}\\|;:'\",.<>/?~`".to_slug(),
            "punctuation-at-and-plus-slash"
        );
        assert_eq!(
            "This: is an example! 😬".to_slug(),
            "this-is-an-example-grimacing"
        );
        assert_eq!("This → that".to_slug(), "this-that");
        assert_eq!("This→that".to_slug(), "this-that");
        assert_eq!(
            "Converting a BGRA &[u8] to RGB [u8;N] (for images)?".to_slug(),
            "converting-a-bgra-and-u8-to-rgb-u8-n-for-images"
        );
        assert_eq!(
            "Condensation with Nitromethane-2".to_slug(),
            "condensation-with-nitromethane-2"
        );
        assert_eq!("What about u8".to_slug(), "what-about-u8");
        assert_eq!("".to_slug(), "");
        assert_eq!("-".to_slug(), "");
        assert_eq!("-2".to_slug(), "2");
        assert_eq!("--2".to_slug(), "2");
        assert_eq!("a--2".to_slug(), "a-2");
        assert_eq!("abc-1-2-3".to_slug(), "abc-1-2-3");
        assert_eq!("abc-2-def".to_slug(), "abc-2-def");
        assert_eq!("What about COVID-19".to_slug(), "what-about-covid-19");
        assert_eq!(
            "1+2+3+... = -1/12 ?".to_slug(),
            "1-plus-2-plus-3-plus-1-slash-12"
        );
        assert_eq!("I'm back".to_slug(), "im-back");
        assert_eq!("'''".to_slug(), "");
        assert_eq!(
            "junk from \"tryptamine carbonate\"".to_slug(),
            "junk-from-tryptamine-carbonate"
        );
        assert_eq!("this & that".to_slug(), "this-and-that");
        assert_eq!("-&-".to_slug(), "and");
        assert_eq!("test@example.com".to_slug(), "test-at-example-com");
        assert_eq!("@@@".to_slug(), "at-at-at");
        assert_eq!("19/22 or 24/40 ???".to_slug(), "19-slash-22-or-24-slash-40");
        assert_eq!(
            "https://www.example.com/".to_slug(),
            "https-slash-slash-www-example-com-slash"
        );
        assert_eq!(
            "Sea water@£60/l at Amazon".to_slug(),
            "sea-water-at-ps60-slash-l-at-amazon"
        );
        assert_eq!(
            "Sea water@€60/l at Amazon".to_slug(),
            "sea-water-at-eur60-slash-l-at-amazon"
        );
        assert_eq!(
            "Sea water@$60/l at Amazon".to_slug(),
            "sea-water-at-60-slash-l-at-amazon"
        );
    }
}
