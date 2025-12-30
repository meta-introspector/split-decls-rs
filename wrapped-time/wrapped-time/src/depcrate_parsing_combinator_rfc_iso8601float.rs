// Generated macro for float (function)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601float {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"float"}
// Dependencies: {}
# [doc = " Parse a floating point number as its integer and optional fractional parts."] # [doc = ""] # [doc = " The number must have two digits before the decimal point. If a decimal point is present, at"] # [doc = " least one digit must follow."] # [doc = ""] # [doc = " The return type is a tuple of the integer part and optional fraction part."] # [inline] pub (crate) fn float (input : & [u8]) -> Option < ParsedItem < '_ , (u8 , Option < f64 >) > > { let ParsedItem (input , integer_part) = match input { [first_digit @ b'0' ..= b'9' , second_digit @ b'0' ..= b'9' , input @ ..] => { ParsedItem (input , (first_digit - b'0') * 10 + (second_digit - b'0')) } _ => return None , } ; if let Some (ParsedItem (input , ())) = decimal_sign (input) { let ParsedItem (mut input , mut fractional_part) = any_digit (input) ? . map (| digit | ((digit - b'0') as f64) / 10.) ; let mut divisor = 10. ; while let Some (ParsedItem (new_input , digit)) = any_digit (input) { input = new_input ; divisor *= 10. ; fractional_part += (digit - b'0') as f64 / divisor ; } Some (ParsedItem (input , (integer_part , Some (fractional_part)))) } else { Some (ParsedItem (input , (integer_part , None))) } }
};
}
