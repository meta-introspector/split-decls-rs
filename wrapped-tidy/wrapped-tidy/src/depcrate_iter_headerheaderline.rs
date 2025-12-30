// Generated macro for HeaderLine (struct)
macro_rules! Depcrate_iter_headerHeaderLine {
() => {
// Module: crate::iter_header
// Provides: {"HeaderLine"}
// Dependencies: {}
# [doc = " A header line, like `//@name: value` consists of the prefix `//@` and the directive"] # [doc = " `name: value`. It is also possibly revisioned, e.g. `//@[revision] name: value`."] pub (crate) struct HeaderLine < 'ln > { pub (crate) line_number : usize , pub (crate) revision : Option < & 'ln str > , pub (crate) directive : & 'ln str , }
};
}
