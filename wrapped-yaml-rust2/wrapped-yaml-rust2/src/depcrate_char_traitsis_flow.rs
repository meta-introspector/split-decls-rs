// Generated macro for is_flow (function)
macro_rules! Depcrate_char_traitsis_flow {
() => {
// Module: crate::char_traits
// Provides: {"is_flow"}
// Dependencies: {}
# [doc = " Check whether the character is a YAML flow character (one of `,[]{}`)."] # [inline] pub (crate) fn is_flow (c : char) -> bool { matches ! (c , ',' | '[' | ']' | '{' | '}') }
};
}
