// Generated macro for impl_18 (impl)
macro_rules! Depcrate_astimpl_18 {
() => {
// Module: crate::ast
// Provides: {"impl_18"}
// Dependencies: {}
impl Word { # [doc = " Creates a new [Word] containing the given [segments](WordSgmt)."] pub (crate) fn new (sgmts : Vec < WordSgmt >) -> Self { Self (sgmts) } # [doc = " Returns a reference to this [Word]'s segments."] pub (crate) fn sgmts (& self) -> & [WordSgmt] { & self . 0 } # [doc = " If this [Word] represents a literal string, returns the literal."] # [doc = ""] # [doc = " A word is considered a literal if:"] # [doc = " - The word has a single segment."] # [doc = " - Such segment is a [WordSgmt::Lit]."] pub (crate) fn as_lit (& self) -> Option < & str > { match & self . 0 . first () { Some (WordSgmt :: Lit (lit)) if self . 0 . len () == 1 => Some (lit) , _ => None , } } }
};
}
