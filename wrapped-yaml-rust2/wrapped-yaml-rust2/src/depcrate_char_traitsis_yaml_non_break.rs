// Generated macro for is_yaml_non_break (function)
macro_rules! Depcrate_char_traitsis_yaml_non_break {
() => {
// Module: crate::char_traits
// Provides: {"is_yaml_non_break"}
// Dependencies: {}
# [doc = " Check whether the character is a YAML non-breaking character."] # [inline] pub (crate) fn is_yaml_non_break (c : char) -> bool { ! is_break (c) && ! is_bom (c) }
};
}
