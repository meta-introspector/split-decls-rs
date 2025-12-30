// Generated macro for impl_108 (impl)
macro_rules! Depcrate_internals_caseimpl_108 {
() => {
// Module: crate::internals::case
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'a > Display for ParseError < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("unknown rename rule `rename_all = ") ? ; Debug :: fmt (self . unknown , f) ? ; f . write_str ("`, expected one of ") ? ; for (i , (name , _rule)) in RENAME_RULES . iter () . enumerate () { if i > 0 { f . write_str (", ") ? ; } Debug :: fmt (name , f) ? ; } Ok (()) } }
};
}
