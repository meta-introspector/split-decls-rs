// Generated macro for impl_234 (impl)
macro_rules! Depcrate_metadataimpl_234 {
() => {
// Module: crate::metadata
// Provides: {"impl_234"}
// Dependencies: {}
impl fmt :: Debug for Kind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Kind(") ? ; let mut has_bits = false ; let mut write_bit = | name : & str | { if has_bits { f . write_str (" | ") ? ; } f . write_str (name) ? ; has_bits = true ; Ok (()) } ; if self . is_event () { write_bit ("EVENT") ? ; } if self . is_span () { write_bit ("SPAN") ? ; } if self . is_hint () { write_bit ("HINT") ? ; } if ! has_bits { write ! (f , "{:#b}" , self . 0) ? ; } f . write_str (")") } }
};
}
