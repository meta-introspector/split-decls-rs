// Generated macro for len_if_padded (function)
macro_rules! Depcrate_consolelen_if_padded {
() => {
// Module: crate::console
// Provides: {"len_if_padded"}
// Dependencies: {}
fn len_if_padded (t : & TestDescAndFn) -> usize { match t . testfn . padding () { NamePadding :: PadNone => 0 , NamePadding :: PadOnRight => t . desc . name . as_slice () . len () , } }
};
}
