// Generated macro for extra_offset (function)
macro_rules! Depcrate_utilsextra_offset {
() => {
// Module: crate::utils
// Provides: {"extra_offset"}
// Dependencies: {}
pub (crate) fn extra_offset (text : & str , shape : Shape) -> usize { match text . rfind ('\n') { Some (idx) => text . len () . saturating_sub (idx + 1 + shape . used_width ()) , None => text . len () , } }
};
}
