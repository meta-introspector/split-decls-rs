// Generated macro for create_unsigned_pattern (function)
macro_rules! Depcrate_percentcreate_unsigned_pattern {
() => {
// Module: crate::percent
// Provides: {"create_unsigned_pattern"}
// Dependencies: {}
# [doc = " Used only for positive percents."] # [doc = " If you need an approximate, explicit plus, or negative percent, use the negative pattern."] fn create_unsigned_pattern < 'a > (pattern : & str , localized_percent_sign : & str ,) -> Result < VarZeroCow < 'a , Pattern < SinglePlaceholder > > , DataError > { let percent_sign_index = pattern . find ('%') . unwrap () ; let first_num_index = pattern . find (['0' , '#']) . unwrap () ; let last_num_index = pattern . rfind (['0' , '#']) . unwrap () ; let percent_prefix = if percent_sign_index == 0 { "" } else if percent_sign_index < first_num_index { & pattern [0 .. percent_sign_index] } else { & pattern [last_num_index + 1 .. percent_sign_index] } ; let percent_suffix = if percent_sign_index == 0 || percent_sign_index < first_num_index { & pattern [1 .. first_num_index] } else { & pattern [percent_sign_index + 1 ..] } ; let percent_symbol = String :: new () + percent_prefix + localized_percent_sign + percent_suffix ; let pattern = SinglePlaceholderPattern :: try_from_str (& if percent_sign_index > first_num_index { "{0}" . to_owned () + & percent_symbol } else { percent_symbol + "{0}" } , Default :: default () ,) . map_err (| e | DataError :: custom ("Could not parse pattern") . with_display_context (& e)) ? ; Ok (VarZeroCow :: new_owned (pattern)) }
};
}
