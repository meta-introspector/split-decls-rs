// Generated macro for is_yaml_non_space (function)
macro_rules! Depcrate_char_traitsis_yaml_non_space {
() => {
// Module: crate::char_traits
// Provides: {"is_yaml_non_space"}
// Dependencies: {}
# [doc = " Check whether the character is NOT a YAML whitespace (` ` / `\\t`)."] # [inline] pub (crate) fn is_yaml_non_space (c : char) -> bool { is_yaml_non_break (c) && ! is_blank (c) }
};
}
