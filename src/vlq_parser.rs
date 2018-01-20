const MAX_VLQ_STRING_LENGTH_FOR_60_BITS: usize = 12;
const MAX_VLQ_STRING_LENGTH_FOR_64_BITS: usize = 13;
const SHIFT_FACTOR: u8 = 5;
const VLQ_LAST_CHAR_MASK: u8 = 0b00100000;
const VLQ_CHAR_VALUE_MASK: u8 = 0b00011111;

#[derive(Debug,PartialEq)]
pub enum VlqParserError {
    VlqInvalidStringLength = 0,
    VlqInvalidStringStart = 1,
    VlqInvalidStringRest = 2,
}

fn is_vlqstring_overflowing(vlq_string: &str) -> bool {
    let fits_in_60_bits = vlq_string.len() <= MAX_VLQ_STRING_LENGTH_FOR_60_BITS;
    let doesnt_fit_in_64_bits = vlq_string.len() > MAX_VLQ_STRING_LENGTH_FOR_64_BITS;

    if fits_in_60_bits {
        return false;
    } else if doesnt_fit_in_64_bits {
        return true;
    } else {
        return vlq_string.chars()
            .nth(0)
            .unwrap() as u8 & 0b10000 != 0;
    }
}

fn is_vlqstring_length_valid(vlq_string: &str) -> bool {
    let fits_in_60_bits = vlq_string.len() <= MAX_VLQ_STRING_LENGTH_FOR_60_BITS;
    let fits_in_64_bits = vlq_string.len() <= MAX_VLQ_STRING_LENGTH_FOR_64_BITS;
    let is_valid_length: bool;

    if !fits_in_60_bits && fits_in_64_bits {
        is_valid_length = !is_vlqstring_overflowing(vlq_string);
    } else {
        is_valid_length = fits_in_60_bits;
    }

    return is_valid_length;
}

fn is_vlqstring_start_valid(vlq_string: &str) -> bool {
    return vlq_string
        .chars()
        .nth(0)
        .map_or(true, |c| is_last_vlq_character(c as u8));
}

fn is_vlqstring_rest_valid(vlq_string: &str) -> bool {
    return vlq_string
        .chars()
        .skip(1)
        .all(|c| !is_last_vlq_character(c as u8));
}

fn validate_vlqstring(vlq_string: &str) -> Result<(), VlqParserError> {
    if !is_vlqstring_length_valid(vlq_string) {
        return Err(VlqParserError::VlqInvalidStringLength);
    }

    if !is_vlqstring_start_valid(vlq_string) {
        return Err(VlqParserError::VlqInvalidStringStart);
    }

    if !is_vlqstring_rest_valid(vlq_string) {
        return Err(VlqParserError::VlqInvalidStringRest);
    }

    return Ok(());
}

fn add_vlq_character(input: u64, vlq_character: u8) -> u64 {
    return (input << SHIFT_FACTOR) | (vlq_character & VLQ_CHAR_VALUE_MASK) as u64;
}

fn is_last_vlq_character(vlq_character: u8) -> bool {
    return (vlq_character & VLQ_LAST_CHAR_MASK) == 0;
}

fn calculate_vlq_value(vlq_string: &str) -> u64 {
    let mut vlq_value: u64 = 0;

    for c in vlq_string.chars() {
        vlq_value = add_vlq_character(vlq_value, c as u8);
    }

    return vlq_value;
}

/// Parses a VLQ (Variable Length Query) encoded number
/// VLQ uses a string for the number format
/// Each characters 5 lowest order bits are digits
/// The last character has bit 6 set to true
///
/// ```
/// use vlq_parser::*;
/// match parse_vlq("") { Ok(result) => assert_eq!(result, 0), Err(e) => assert!(false, e)};
/// match parse_vlq("A") { Ok(result) => assert_eq!(result, 1), Err(e) => assert!(false, e)};
/// match parse_vlq("Aa") { Ok(result) => assert_eq!(result, 33), Err(e) => assert!(false, e)};
/// ```
pub fn parse_vlq(vlq_string: &str) -> Result<u64, VlqParserError> {
    match validate_vlqstring(vlq_string) {
        Ok(_) => Ok(calculate_vlq_value(vlq_string)),
        Err(e) => Err(e)
    }
}
