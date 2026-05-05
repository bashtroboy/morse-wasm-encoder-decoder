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
mod tests;
