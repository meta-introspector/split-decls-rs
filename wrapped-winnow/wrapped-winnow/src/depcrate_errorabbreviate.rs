// Generated macro for abbreviate (function)
macro_rules! Depcrate_errorabbreviate {
() => {
// Module: crate::error
// Provides: {"abbreviate"}
// Dependencies: {}
# [cfg (feature = "std")] fn abbreviate (input : String) -> String { let mut abbrev = None ; if let Some ((line , _)) = input . split_once ('\n') { abbrev = Some (line) ; } let max_len = 20 ; let current = abbrev . unwrap_or (& input) ; if max_len < current . len () { if let Some ((index , _)) = current . char_indices () . nth (max_len) { abbrev = Some (& current [.. index]) ; } } if let Some (abbrev) = abbrev { format ! ("{abbrev}...") } else { input } }
};
}
