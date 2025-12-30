// Generated macro for strip_separated_padded_characters (function)
macro_rules! Depcrate_durationstrip_separated_padded_characters {
() => {
// Module: crate::duration
// Provides: {"strip_separated_padded_characters"}
// Dependencies: {}
# [doc = " Strips the passed padded characters separated by a uniform separator."] # [doc = " Returns the padding sizes and the separator."] # [doc = " Modifies `s` to point to the remaining string."] # [doc = " Returns an error if the separator is missing or inconsistent."] fn strip_separated_padded_characters < 's , const N : usize > (string : & mut & 's str , chars : [char ; N] ,) -> Result < ([u8 ; N] , & 's str) , DataError > { let mut padding = [0u8 ; N] ; let mut sep = None ; for i in 0 .. (N - 1) { padding [i] += strip_padded_character (string , chars [i]) ; let (curr_sep , rest) = string . split_once (chars [i + 1]) . ok_or_else (| | DataError :: custom ("Missing separator in pattern")) ? ; padding [i + 1] += 1 ; * string = rest ; match sep { Some (sep) => { if sep != curr_sep { return Err (DataError :: custom ("Inconsistent separators in pattern")) ; } } None => sep = Some (curr_sep) , } } padding [N - 1] += strip_padded_character (string , chars [N - 1]) ; if ! string . is_empty () { return Err (DataError :: custom ("Unexpected characters in duration patterns" ,)) ; } Ok ((padding , sep . ok_or_else (| | DataError :: custom ("Missing separator in pattern")) ? ,)) }
};
}
