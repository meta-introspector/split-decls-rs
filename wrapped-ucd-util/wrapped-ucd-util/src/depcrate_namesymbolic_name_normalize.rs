// Generated macro for symbolic_name_normalize (function)
macro_rules! Depcrate_namesymbolic_name_normalize {
() => {
// Module: crate::name
// Provides: {"symbolic_name_normalize"}
// Dependencies: {}
# [doc = " Normalize the given symbolic name in place according to UAX44-LM3."] # [doc = ""] # [doc = " A \"symbolic name\" typically corresponds to property names and property"] # [doc = " value aliases. Note, though, that it should not be applied to property"] # [doc = " string values."] # [doc = ""] # [doc = " See: https://unicode.org/reports/tr44/#UAX44-LM2"] pub fn symbolic_name_normalize (string : & mut String) { let bytes = unsafe { string . as_mut_vec () } ; let len = symbolic_name_normalize_bytes (bytes) . len () ; bytes . truncate (len) ; }
};
}
