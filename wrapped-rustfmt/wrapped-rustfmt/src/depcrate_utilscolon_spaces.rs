// Generated macro for colon_spaces (function)
macro_rules! Depcrate_utilscolon_spaces {
() => {
// Module: crate::utils
// Provides: {"colon_spaces"}
// Dependencies: {}
# [inline] pub (crate) fn colon_spaces (config : & Config) -> & 'static str { let before = config . space_before_colon () ; let after = config . space_after_colon () ; match (before , after) { (true , true) => " : " , (true , false) => " :" , (false , true) => ": " , (false , false) => ":" , } }
};
}
