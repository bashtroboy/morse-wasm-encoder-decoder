use wasm_bindgen::prelude::*;

const ALPHABET: &[(char, &str)] = &[
    ('A', ".-"),    ('B', "-..."),  ('C', "-.-."),  ('D', "-.."),   ('E', "."),
    ('F', "..-."),  ('G', "--."),   ('H', "...."),  ('I', ".."),    ('J', ".---"),
    ('K', "-.-"),   ('L', ".-.."),  ('M', "--"),    ('N', "-."),    ('O', "---"),
    ('P', ".--."),  ('Q', "--.-"),  ('R', ".-."),   ('S', "..."),   ('T', "-"),
    ('U', "..-"),   ('V', "...-"),  ('W', ".--"),   ('X', "-..-"),  ('Y', "-.--"),
    ('Z', "--.."),
    ('0', "-----"), ('1', ".----"), ('2', "..---"), ('3', "...--"), ('4', "....-"),
    ('5', "....."), ('6', "-...."), ('7', "--..."), ('8', "---.."), ('9', "----."),
    ('.', ".-.-.-"), (',', "--..--"), ('?', "..--.."), ('\'', ".----."),
    ('!', "-.-.--"), ('/', "-..-."),  ('(', "-.--."),  (')', "-.--.-"),
    ('&', ".-..."),  (':', "---..."), (';', "-.-.-."), ('=', "-...-"),
    ('+', ".-.-."),  ('-', "-....-"), ('_', "..--.-"), ('"', ".-..-."),
    ('@', ".--.-."),
];

fn char_to_morse(c: char) -> Option<&'static str> {
    ALPHABET.iter().find(|(ch, _)| *ch == c).map(|(_, m)| *m)
}

fn morse_to_char(m: &str) -> Option<char> {
    ALPHABET.iter().find(|(_, code)| *code == m).map(|(ch, _)| *ch)
}

#[wasm_bindgen]
pub fn encode(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            word.chars()
                .filter_map(|c| char_to_morse(c.to_ascii_uppercase()))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join(" / ")
}

#[wasm_bindgen]
pub fn decode(morse: &str) -> String {
    morse
        .split('/')
        .map(|word| {
            word.split_whitespace()
                .filter_map(morse_to_char)
                .collect::<String>()
        })
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_hello_world() {
        assert_eq!(
            encode("hello world"),
            ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."
        );
    }

    #[test]
    fn decodes_hello_world() {
        assert_eq!(
            decode(".... . .-.. .-.. --- / .-- --- .-. .-.. -.."),
            "HELLO WORLD"
        );
    }

    #[test]
    fn round_trips_text() {
        let original = "The Quick Brown Fox 1234567890";
        let encoded = encode(original);
        assert_eq!(decode(&encoded), original.to_uppercase());
    }

    #[test]
    fn handles_punctuation() {
        assert_eq!(encode("AB?"), ".- -... ..--..");
        assert_eq!(decode(".- -... ..--.."), "AB?");
    }

    #[test]
    fn skips_unknown_chars() {
        assert_eq!(encode("a~b"), ".- -...");
    }

    #[test]
    fn collapses_whitespace() {
        assert_eq!(encode("a   b"), ".- / -...");
    }

    #[test]
    fn slash_character_round_trips() {
        // The character '/' (encoded as -..-.) must not be confused with the
        // ' / ' word separator. Internal slashes stay inside one word.
        assert_eq!(encode("a/b"), ".- -..-. -...");
        assert_eq!(decode(".- -..-. -..."), "A/B");
    }
}
