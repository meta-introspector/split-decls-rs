// Generated macro for impl_33 (impl)
macro_rules! Depcrate_wordimpl_33 {
() => {
// Module: crate::word
// Provides: {"impl_33"}
// Dependencies: {}
impl Word { # [doc = " Creates a new [Word] containing the given [segments](WordSgmt)."] pub fn new (sgmts : Vec < WordSgmt >) -> Self { Self (sgmts) } # [doc = " Returns a reference to this [Word]'s segments."] pub fn sgmts (& self) -> & [WordSgmt] { & self . 0 } # [doc = " If this [Word] represents a literal string, returns the literal."] # [doc = ""] # [doc = " A word is considered a literal if:"] # [doc = " - The word has a single segment."] # [doc = " - Such segment is a [WordSgmt::Lit]."] pub fn as_lit (& self) -> Option < & Spanned < String > > { match & self . 0 . first () { Some (WordSgmt :: Lit (lit)) if self . 0 . len () == 1 => Some (lit) , _ => None , } } }
};
}
