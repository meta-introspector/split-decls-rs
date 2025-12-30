// Generated macro for impl_55 (impl)
macro_rules! Depcrate_legacyimpl_55 {
() => {
// Module: crate::legacy
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'tcx > PrettyPrinter < 'tcx > for LegacySymbolMangler < 'tcx > { fn should_print_optional_region (& self , _region : ty :: Region < '_ >) -> bool { false } fn comma_sep < T > (& mut self , mut elems : impl Iterator < Item = T >) -> Result < () , PrintError > where T : Print < 'tcx , Self > , { if let Some (first) = elems . next () { first . print (self) ? ; for elem in elems { self . write_str (",") ? ; elem . print (self) ? ; } } Ok (()) } fn generic_delimiters (& mut self , f : impl FnOnce (& mut Self) -> Result < () , PrintError > ,) -> Result < () , PrintError > { write ! (self , "<") ? ; let kept_within_component = mem :: replace (& mut self . keep_within_component , true) ; f (self) ? ; self . keep_within_component = kept_within_component ; write ! (self , ">") ? ; Ok (()) } }
};
}
