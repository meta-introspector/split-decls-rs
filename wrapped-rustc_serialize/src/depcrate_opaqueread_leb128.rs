// Generated macro for read_leb128 (macro)
macro_rules! Depcrate_opaqueread_leb128 {
() => {
// Module: crate::opaque
// Provides: {"read_leb128"}
// Dependencies: {}
macro_rules ! read_leb128 { ($ this_fn : ident , $ int_ty : ty , $ read_leb_fn : ident) => { # [inline] fn $ this_fn (& mut self) -> $ int_ty { leb128 ::$ read_leb_fn (self) } } ; }
};
}
