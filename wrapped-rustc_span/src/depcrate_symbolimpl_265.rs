// Generated macro for impl_265 (impl)
macro_rules! Depcrate_symbolimpl_265 {
() => {
// Module: crate::symbol
// Provides: {"impl_265"}
// Dependencies: {}
impl fmt :: Display for IdentPrinter { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let s = match self . mode { IdentPrintMode :: Normal if self . symbol == kw :: DollarCrate && let Some (span) = self . convert_dollar_crate => { let converted = span . ctxt () . dollar_crate_name () ; if ! converted . is_path_segment_keyword () { f . write_str ("::") ? ; } converted } IdentPrintMode :: Normal => self . symbol , IdentPrintMode :: RawIdent => { f . write_str ("r#") ? ; self . symbol } IdentPrintMode :: RawLifetime => { f . write_str ("'r#") ? ; let s = self . symbol . as_str () . strip_prefix ("'") . expect ("only lifetime idents should be passed with RawLifetime mode") ; Symbol :: intern (s) } } ; s . fmt (f) } }
};
}
