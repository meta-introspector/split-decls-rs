// Generated macro for impl_88 (impl)
macro_rules! Depcrate_parseimpl_88 {
() => {
// Module: crate::parse
// Provides: {"impl_88"}
// Dependencies: {}
impl FromStr for Comparator { type Err = Error ; fn from_str (text : & str) -> Result < Self , Self :: Err > { let text = text . trim_start_matches (' ') ; let (comparator , pos , rest) = comparator (text) ? ; if ! rest . is_empty () { let unexpected = rest . chars () . next () . unwrap () ; return Err (Error :: new (ErrorKind :: UnexpectedCharAfter (pos , unexpected))) ; } Ok (comparator) } }
};
}
