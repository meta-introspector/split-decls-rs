// Generated macro for LineInfo (struct)
macro_rules! Depcrate_tyLineInfo {
() => {
// Module: crate::ty
// Provides: {"LineInfo"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , Serialize)] # [doc = " Information you get from `Span` in a struct form."] # [doc = " Line and col start from 1."] pub struct LineInfo { pub start_line : usize , pub start_col : usize , pub end_line : usize , pub end_col : usize , }
};
}
