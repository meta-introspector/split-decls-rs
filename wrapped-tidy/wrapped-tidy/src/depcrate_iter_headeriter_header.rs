// Generated macro for iter_header (function)
macro_rules! Depcrate_iter_headeriter_header {
() => {
// Module: crate::iter_header
// Provides: {"iter_header"}
// Dependencies: {}
# [doc = " Iterate through compiletest headers in a test contents."] # [doc = ""] # [doc = " Adjusted from compiletest/src/header.rs."] pub (crate) fn iter_header < 'ln > (contents : & 'ln str , it : & mut dyn FnMut (HeaderLine < 'ln >)) { for (line_number , ln) in (1 ..) . zip (contents . lines ()) { let ln = ln . trim () ; let Some (remainder) = ln . strip_prefix (COMMENT) else { continue ; } ; if let Some (remainder) = remainder . trim_start () . strip_prefix ('[') { let Some ((revision , remainder)) = remainder . split_once (']') else { panic ! ("malformed revision directive: expected `//@[rev]`, found `{ln}`") ; } ; it (HeaderLine { line_number , revision : Some (revision) , directive : remainder . trim () }) ; } else { it (HeaderLine { line_number , revision : None , directive : remainder . trim () }) ; } } }
};
}
