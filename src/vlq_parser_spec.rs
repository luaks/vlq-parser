use vlq_parser::*;

macro_rules! vlq_parse_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        #[allow(non_snake_case)]
        fn $name() {
            // given
            let (input, expected) = $value;

            // when
            match parse_vlq(input) {
                Ok(result) => {
                    // then
                    assert_eq!(result, expected);
                    return ();
                },
                Err(_) => assert_eq!(0, 1)
            };
        }
    )*
    }
}

vlq_parse_tests! {
    empty_string_to_0: ("", 0),
    string__at__to_0: ("@", 0),
    string__at_3xgravis_to_0: ("@```", 0),
    string_A_to_1: ("A", 1),
    string_B_to_2: ("B", 2),
    string_C_to_3: ("C", 3),
    string_Z_to_26: ("Z", 26),
    string__underscore__to_31: ("_", 31),
    string_Aa_to_33: ("Aa", 33),
    string_A_at__to_32: ("A`", 32),
    string_A_DEL__to_63: ("A\x7f", 63),
    string_match_int_max: ("O\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f", 0xFFFFFFFFFFFFFFFF),
}

#[test]
fn ensure_too_long_vlq_string_is_caught() {
    // given
    let input = "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    // when
    match parse_vlq(input) {
        Ok(_) => assert!(false, "error for vql string to long should have been thrown"),
        Err(e) => assert_eq!(e, VlqParserError::VlqInvalidStringLength)
    }
}

#[test]
fn ensure_invalid_start_is_caught() {
    // given
    let input = "a";

    // when
    match parse_vlq(input) {
        Ok(_) => assert!(false, "error for invalid start should have been thrown"),
        Err(e) => assert_eq!(e, VlqParserError::VlqInvalidStringStart)
    }
}

#[test]
fn ensure_invalid_rest_is_caught() {
    // given
    let input = "AaA";

    // when
    match parse_vlq(input) {
        Ok(_) => assert!(false, "error for invalid rest should have been thrown"),
        Err(e) => assert_eq!(e, VlqParserError::VlqInvalidStringRest)
    }
}

