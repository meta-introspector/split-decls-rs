// Generated macro for is_raw_identifier (function)
macro_rules! Depcrate_utilsis_raw_identifier {
() => {
// Module: crate::utils
// Provides: {"is_raw_identifier"}
// Dependencies: {}
# [inline] pub fn is_raw_identifier (name : & str , edition : parser :: Edition) -> bool { let is_keyword = SyntaxKind :: from_keyword (name , edition) . is_some () ; is_keyword && ! matches ! (name , "self" | "crate" | "super" | "Self") }
};
}
