// Generated macro for slice_tails (function)
macro_rules! Depcrateslice_tails {
() => {
// Module: crate
// Provides: {"slice_tails"}
// Dependencies: {}
# [doc = " Returns all final segments of the argument, longest first."] pub fn slice_tails < T > (this : & [T]) -> impl Iterator < Item = & [T] > { (0 .. this . len ()) . map (| i | & this [i ..]) }
};
}
