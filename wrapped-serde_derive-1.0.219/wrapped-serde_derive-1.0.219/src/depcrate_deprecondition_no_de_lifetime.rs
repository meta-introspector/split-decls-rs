// Generated macro for precondition_no_de_lifetime (function)
macro_rules! Depcrate_deprecondition_no_de_lifetime {
() => {
// Module: crate::de
// Provides: {"precondition_no_de_lifetime"}
// Dependencies: {}
fn precondition_no_de_lifetime (cx : & Ctxt , cont : & Container) { if let BorrowedLifetimes :: Borrowed (_) = borrowed_lifetimes (cont) { for param in cont . generics . lifetimes () { if param . lifetime . to_string () == "'de" { cx . error_spanned_by (& param . lifetime , "cannot deserialize when there is a lifetime parameter called 'de" ,) ; return ; } } } }
};
}
