// Generated macro for is_anchor_char (function)
macro_rules! Depcrate_char_traitsis_anchor_char {
() => {
// Module: crate::char_traits
// Provides: {"is_anchor_char"}
// Dependencies: {}
# [doc = " Check whether the character is a valid YAML anchor name character."] # [inline] pub (crate) fn is_anchor_char (c : char) -> bool { is_yaml_non_space (c) && ! is_flow (c) && ! is_z (c) }
};
}
