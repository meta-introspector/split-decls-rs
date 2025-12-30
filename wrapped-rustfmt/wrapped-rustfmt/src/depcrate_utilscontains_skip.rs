// Generated macro for contains_skip (function)
macro_rules! Depcrate_utilscontains_skip {
() => {
// Module: crate::utils
// Provides: {"contains_skip"}
// Dependencies: {}
# [inline] pub (crate) fn contains_skip (attrs : & [Attribute]) -> bool { attrs . iter () . any (| a | a . meta () . map_or (false , | a | is_skip (& a))) }
};
}
