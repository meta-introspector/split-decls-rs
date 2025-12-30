// Generated macro for truncate (function)
macro_rules! Depcrate_builtins_filters_stringtruncate {
() => {
// Module: crate::builtins::filters::string
// Provides: {"truncate"}
// Dependencies: {}
# [doc = " Truncates a string to the indicated length."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `value`   - The string that needs to be truncated."] # [doc = " * `args`    - A set of key/value arguments that can take the following"] # [doc = "   keys."] # [doc = " * `length`  - The length at which the string needs to be truncated. If"] # [doc = "   the length is larger than the length of the string, the string is"] # [doc = "   returned untouched. The default value is 255."] # [doc = " * `end`     - The ellipsis string to be used if the given string is"] # [doc = "   truncated. The default value is \"…\"."] # [doc = ""] # [doc = " # Remarks"] # [doc = ""] # [doc = " The return value of this function might be longer than `length`: the `end`"] # [doc = " string is *added* after the truncation occurs."] # [doc = ""] pub fn truncate (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("truncate" , "value" , String , value) ; let length = match args . get ("length") { Some (l) => try_get_value ! ("truncate" , "length" , usize , l) , None => 255 , } ; let end = match args . get ("end") { Some (l) => try_get_value ! ("truncate" , "end" , String , l) , None => "…" . to_string () , } ; let graphemes = s . grapheme_indices (true) . collect :: < Vec < (usize , & str) > > () ; if length >= graphemes . len () { return Ok (to_value (& s) . unwrap ()) ; } let result = s [.. graphemes [length] . 0] . to_string () + & end ; Ok (to_value (result) . unwrap ()) }
};
}
