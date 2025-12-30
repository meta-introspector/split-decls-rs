// Generated macro for impl_6 (impl)
macro_rules! Depcrate_displayimpl_6 {
() => {
// Module: crate::display
// Provides: {"impl_6"}
// Dependencies: {}
impl Display for VersionReq { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { if self . comparators . is_empty () { return formatter . write_str ("*") ; } for (i , comparator) in self . comparators . iter () . enumerate () { if i > 0 { formatter . write_str (", ") ? ; } write ! (formatter , "{}" , comparator) ? ; } Ok (()) } }
};
}
