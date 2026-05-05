use super::*;

// ===========================================
// Basic encoding/decoding tests
// ===========================================

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

// ===========================================
// Individual letter encoding tests
// ===========================================

#[test]
fn encodes_letter_a() {
    assert_eq!(encode("A"), ".-");
}

#[test]
fn encodes_letter_b() {
    assert_eq!(encode("B"), "-...");
}

#[test]
fn encodes_letter_c() {
    assert_eq!(encode("C"), "-.-.");
}

#[test]
fn encodes_letter_d() {
    assert_eq!(encode("D"), "-..");
}

#[test]
fn encodes_letter_e() {
    assert_eq!(encode("E"), ".");
}

#[test]
fn encodes_letter_f() {
    assert_eq!(encode("F"), "..-.");
}

#[test]
fn encodes_letter_g() {
    assert_eq!(encode("G"), "--.");
}

#[test]
fn encodes_letter_h() {
    assert_eq!(encode("H"), "....");
}

#[test]
fn encodes_letter_i() {
    assert_eq!(encode("I"), "..");
}

#[test]
fn encodes_letter_j() {
    assert_eq!(encode("J"), ".---");
}

#[test]
fn encodes_letter_k() {
    assert_eq!(encode("K"), "-.-");
}

#[test]
fn encodes_letter_l() {
    assert_eq!(encode("L"), ".-..");
}

#[test]
fn encodes_letter_m() {
    assert_eq!(encode("M"), "--");
}

#[test]
fn encodes_letter_n() {
    assert_eq!(encode("N"), "-.");
}

#[test]
fn encodes_letter_o() {
    assert_eq!(encode("O"), "---");
}

#[test]
fn encodes_letter_p() {
    assert_eq!(encode("P"), ".--.");
}

#[test]
fn encodes_letter_q() {
    assert_eq!(encode("Q"), "--.-");
}

#[test]
fn encodes_letter_r() {
    assert_eq!(encode("R"), ".-.");
}

#[test]
fn encodes_letter_s() {
    assert_eq!(encode("S"), "...");
}

#[test]
fn encodes_letter_t() {
    assert_eq!(encode("T"), "-");
}

#[test]
fn encodes_letter_u() {
    assert_eq!(encode("U"), "..-");
}

#[test]
fn encodes_letter_v() {
    assert_eq!(encode("V"), "...-");
}

#[test]
fn encodes_letter_w() {
    assert_eq!(encode("W"), ".--");
}

#[test]
fn encodes_letter_x() {
    assert_eq!(encode("X"), "-..-");
}

#[test]
fn encodes_letter_y() {
    assert_eq!(encode("Y"), "-.--");
}

#[test]
fn encodes_letter_z() {
    assert_eq!(encode("Z"), "--..");
}

// ===========================================
// Individual letter decoding tests
// ===========================================

#[test]
fn decodes_letter_a() {
    assert_eq!(decode(".-"), "A");
}

#[test]
fn decodes_letter_b() {
    assert_eq!(decode("-..."), "B");
}

#[test]
fn decodes_letter_c() {
    assert_eq!(decode("-.-."), "C");
}

#[test]
fn decodes_letter_d() {
    assert_eq!(decode("-.."), "D");
}

#[test]
fn decodes_letter_e() {
    assert_eq!(decode("."), "E");
}

#[test]
fn decodes_letter_f() {
    assert_eq!(decode("..-."), "F");
}

#[test]
fn decodes_letter_g() {
    assert_eq!(decode("--."), "G");
}

#[test]
fn decodes_letter_h() {
    assert_eq!(decode("...."), "H");
}

#[test]
fn decodes_letter_i() {
    assert_eq!(decode(".."), "I");
}

#[test]
fn decodes_letter_j() {
    assert_eq!(decode(".---"), "J");
}

#[test]
fn decodes_letter_k() {
    assert_eq!(decode("-.-"), "K");
}

#[test]
fn decodes_letter_l() {
    assert_eq!(decode(".-.."), "L");
}

#[test]
fn decodes_letter_m() {
    assert_eq!(decode("--"), "M");
}

#[test]
fn decodes_letter_n() {
    assert_eq!(decode("-."), "N");
}

#[test]
fn decodes_letter_o() {
    assert_eq!(decode("---"), "O");
}

#[test]
fn decodes_letter_p() {
    assert_eq!(decode(".--."), "P");
}

#[test]
fn decodes_letter_q() {
    assert_eq!(decode("--.-"), "Q");
}

#[test]
fn decodes_letter_r() {
    assert_eq!(decode(".-."), "R");
}

#[test]
fn decodes_letter_s() {
    assert_eq!(decode("..."), "S");
}

#[test]
fn decodes_letter_t() {
    assert_eq!(decode("-"), "T");
}

#[test]
fn decodes_letter_u() {
    assert_eq!(decode("..-"), "U");
}

#[test]
fn decodes_letter_v() {
    assert_eq!(decode("...-"), "V");
}

#[test]
fn decodes_letter_w() {
    assert_eq!(decode(".--"), "W");
}

#[test]
fn decodes_letter_x() {
    assert_eq!(decode("-..-"), "X");
}

#[test]
fn decodes_letter_y() {
    assert_eq!(decode("-.--"), "Y");
}

#[test]
fn decodes_letter_z() {
    assert_eq!(decode("--.."), "Z");
}

// ===========================================
// Number encoding tests
// ===========================================

#[test]
fn encodes_number_0() {
    assert_eq!(encode("0"), "-----");
}

#[test]
fn encodes_number_1() {
    assert_eq!(encode("1"), ".----");
}

#[test]
fn encodes_number_2() {
    assert_eq!(encode("2"), "..---");
}

#[test]
fn encodes_number_3() {
    assert_eq!(encode("3"), "...--");
}

#[test]
fn encodes_number_4() {
    assert_eq!(encode("4"), "....-");
}

#[test]
fn encodes_number_5() {
    assert_eq!(encode("5"), ".....");
}

#[test]
fn encodes_number_6() {
    assert_eq!(encode("6"), "-....");
}

#[test]
fn encodes_number_7() {
    assert_eq!(encode("7"), "--...");
}

#[test]
fn encodes_number_8() {
    assert_eq!(encode("8"), "---..");
}

#[test]
fn encodes_number_9() {
    assert_eq!(encode("9"), "----.");
}

// ===========================================
// Number decoding tests
// ===========================================

#[test]
fn decodes_number_0() {
    assert_eq!(decode("-----"), "0");
}

#[test]
fn decodes_number_1() {
    assert_eq!(decode(".----"), "1");
}

#[test]
fn decodes_number_2() {
    assert_eq!(decode("..---"), "2");
}

#[test]
fn decodes_number_3() {
    assert_eq!(decode("...--"), "3");
}

#[test]
fn decodes_number_4() {
    assert_eq!(decode("....-"), "4");
}

#[test]
fn decodes_number_5() {
    assert_eq!(decode("....."), "5");
}

#[test]
fn decodes_number_6() {
    assert_eq!(decode("-...."), "6");
}

#[test]
fn decodes_number_7() {
    assert_eq!(decode("--..."), "7");
}

#[test]
fn decodes_number_8() {
    assert_eq!(decode("---.."), "8");
}

#[test]
fn decodes_number_9() {
    assert_eq!(decode("----."), "9");
}

// ===========================================
// Punctuation encoding tests
// ===========================================

#[test]
fn encodes_period() {
    assert_eq!(encode("."), ".-.-.-");
}

#[test]
fn encodes_comma() {
    assert_eq!(encode(","), "--..--");
}

#[test]
fn encodes_question_mark() {
    assert_eq!(encode("?"), "..--..");
}

#[test]
fn encodes_apostrophe() {
    assert_eq!(encode("'"), ".----.");
}

#[test]
fn encodes_exclamation() {
    assert_eq!(encode("!"), "-.-.--");
}

#[test]
fn encodes_forward_slash() {
    assert_eq!(encode("/"), "-..-.");
}

#[test]
fn encodes_open_paren() {
    assert_eq!(encode("("), "-.--.");
}

#[test]
fn encodes_close_paren() {
    assert_eq!(encode(")"), "-.--.-");
}

#[test]
fn encodes_ampersand() {
    assert_eq!(encode("&"), ".-...");
}

#[test]
fn encodes_colon() {
    assert_eq!(encode(":"), "---...");
}

#[test]
fn encodes_semicolon() {
    assert_eq!(encode(";"), "-.-.-.");
}

#[test]
fn encodes_equals() {
    assert_eq!(encode("="), "-...-");
}

#[test]
fn encodes_plus() {
    assert_eq!(encode("+"), ".-.-.");
}

#[test]
fn encodes_minus() {
    assert_eq!(encode("-"), "-....-");
}

#[test]
fn encodes_underscore() {
    assert_eq!(encode("_"), "..--.-");
}

#[test]
fn encodes_double_quote() {
    assert_eq!(encode("\""), ".-..-.");
}

#[test]
fn encodes_at_sign() {
    assert_eq!(encode("@"), ".--.-.");
}

// ===========================================
// Punctuation decoding tests
// ===========================================

#[test]
fn decodes_period() {
    assert_eq!(decode(".-.-.-"), ".");
}

#[test]
fn decodes_comma() {
    assert_eq!(decode("--..--"), ",");
}

#[test]
fn decodes_question_mark() {
    assert_eq!(decode("..--.."), "?");
}

#[test]
fn decodes_apostrophe() {
    assert_eq!(decode(".----."), "'");
}

#[test]
fn decodes_exclamation() {
    assert_eq!(decode("-.-.--"), "!");
}

#[test]
fn decodes_forward_slash() {
    assert_eq!(decode("-..-."), "/");
}

#[test]
fn decodes_open_paren() {
    assert_eq!(decode("-.--."), "(");
}

#[test]
fn decodes_close_paren() {
    assert_eq!(decode("-.--.-"), ")");
}

#[test]
fn decodes_ampersand() {
    assert_eq!(decode(".-..."), "&");
}

#[test]
fn decodes_colon() {
    assert_eq!(decode("---..."), ":");
}

#[test]
fn decodes_semicolon() {
    assert_eq!(decode("-.-.-."), ";");
}

#[test]
fn decodes_equals() {
    assert_eq!(decode("-...-"), "=");
}

#[test]
fn decodes_plus() {
    assert_eq!(decode(".-.-."), "+");
}

#[test]
fn decodes_minus() {
    assert_eq!(decode("-....-"), "-");
}

#[test]
fn decodes_underscore() {
    assert_eq!(decode("..--.-"), "_");
}

#[test]
fn decodes_double_quote() {
    assert_eq!(decode(".-..-."), "\"");
}

#[test]
fn decodes_at_sign() {
    assert_eq!(decode(".--.-."), "@");
}

// ===========================================
// Edge case tests
// ===========================================

#[test]
fn handles_empty_string() {
    assert_eq!(encode(""), "");
    assert_eq!(decode(""), "");
}

#[test]
fn handles_only_whitespace() {
    assert_eq!(encode("   "), "");
    assert_eq!(encode("\t\n"), "");
}

#[test]
fn handles_mixed_case() {
    assert_eq!(encode("AbCd"), encode("ABCD"));
    assert_eq!(encode("abcd"), encode("ABCD"));
}

#[test]
fn handles_multiple_words() {
    assert_eq!(encode("a b c"), ".- / -... / -.-.");
}

#[test]
fn handles_leading_trailing_whitespace() {
    assert_eq!(encode("  hello  "), ".... . .-.. .-.. ---");
}

#[test]
fn handles_tabs_and_newlines() {
    assert_eq!(encode("a\tb\nc"), ".- / -... / -.-.");
}

#[test]
fn decodes_extra_spaces_between_letters() {
    assert_eq!(decode(".-  -..."), "AB");
}

#[test]
fn decodes_extra_spaces_around_word_separator() {
    assert_eq!(decode(".-  /  -..."), "A B");
}

#[test]
fn handles_unknown_morse_code() {
    // Unknown morse codes should be skipped
    assert_eq!(decode(".- .------- -..."), "AB");
}

#[test]
fn handles_multiple_slashes() {
    assert_eq!(decode(".- / / -..."), "A B");
}

// ===========================================
// Round-trip tests for various inputs
// ===========================================

#[test]
fn round_trips_alphabet() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let encoded = encode(alphabet);
    assert_eq!(decode(&encoded), alphabet);
}

#[test]
fn round_trips_numbers() {
    let numbers = "0123456789";
    let encoded = encode(numbers);
    assert_eq!(decode(&encoded), numbers);
}

#[test]
fn round_trips_sentence() {
    let sentence = "SOS WE NEED HELP";
    let encoded = encode(sentence);
    assert_eq!(decode(&encoded), sentence);
}

#[test]
fn round_trips_mixed_alphanumeric() {
    let mixed = "ABC123XYZ";
    let encoded = encode(mixed);
    assert_eq!(decode(&encoded), mixed);
}

#[test]
fn round_trips_with_punctuation() {
    let text = "HELLO, WORLD!";
    let encoded = encode(text);
    assert_eq!(decode(&encoded), text);
}

#[test]
fn round_trips_complex_sentence() {
    let sentence = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
    let encoded = encode(sentence);
    assert_eq!(decode(&encoded), sentence);
}

// ===========================================
// Internal function tests
// ===========================================

#[test]
fn char_to_morse_finds_letters() {
    assert_eq!(char_to_morse('A'), Some(".-"));
    assert_eq!(char_to_morse('Z'), Some("--.."));
}

#[test]
fn char_to_morse_finds_numbers() {
    assert_eq!(char_to_morse('0'), Some("-----"));
    assert_eq!(char_to_morse('9'), Some("----."));
}

#[test]
fn char_to_morse_returns_none_for_unknown() {
    assert_eq!(char_to_morse('~'), None);
    assert_eq!(char_to_morse('#'), None);
    assert_eq!(char_to_morse('$'), None);
}

#[test]
fn morse_to_char_finds_letters() {
    assert_eq!(morse_to_char(".-"), Some('A'));
    assert_eq!(morse_to_char("--.."), Some('Z'));
}

#[test]
fn morse_to_char_finds_numbers() {
    assert_eq!(morse_to_char("-----"), Some('0'));
    assert_eq!(morse_to_char("----."), Some('9'));
}

#[test]
fn morse_to_char_returns_none_for_unknown() {
    assert_eq!(morse_to_char("........"), None);
    assert_eq!(morse_to_char("invalid"), None);
}

// ===========================================
// SOS and common phrases
// ===========================================

#[test]
fn encodes_sos() {
    assert_eq!(encode("SOS"), "... --- ...");
}

#[test]
fn decodes_sos() {
    assert_eq!(decode("... --- ..."), "SOS");
}

#[test]
fn encodes_mayday() {
    assert_eq!(encode("MAYDAY"), "-- .- -.-- -.. .- -.--");
}

#[test]
fn encodes_cq() {
    // CQ is a call sign meaning "calling all stations"
    assert_eq!(encode("CQ CQ CQ"), "-.-. --.- / -.-. --.- / -.-. --.-");
}

// ===========================================
// Stress tests
// ===========================================

#[test]
fn handles_long_input() {
    let long_text = "A".repeat(100);
    let encoded = encode(&long_text);
    let decoded = decode(&encoded);
    assert_eq!(decoded, long_text);
}

#[test]
fn handles_many_words() {
    let many_words = vec!["WORD"; 50].join(" ");
    let encoded = encode(&many_words);
    let decoded = decode(&encoded);
    assert_eq!(decoded, many_words);
}

#[test]
fn handles_alternating_characters() {
    let alternating = "ABABABABAB";
    let encoded = encode(alternating);
    assert_eq!(encoded, ".- -... .- -... .- -... .- -... .- -...");
    assert_eq!(decode(&encoded), alternating);
}
