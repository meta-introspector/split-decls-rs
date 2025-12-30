// Generated macro for span_with_attrs_lo_hi (macro)
macro_rules! Depcrate_spannedspan_with_attrs_lo_hi {
() => {
// Module: crate::spanned
// Provides: {"span_with_attrs_lo_hi"}
// Dependencies: {}
macro_rules ! span_with_attrs_lo_hi { ($ this : ident , $ lo : expr , $ hi : expr) => { { let attrs = outer_attributes (&$ this . attrs) ; if attrs . is_empty () { mk_sp ($ lo , $ hi) } else { mk_sp (attrs [0] . span . lo () , $ hi) } } } ; }
};
}
