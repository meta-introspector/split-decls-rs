// Generated macro for write_leb128 (macro)
macro_rules! Depcrate_opaquewrite_leb128 {
() => {
// Module: crate::opaque
// Provides: {"write_leb128"}
// Dependencies: {}
macro_rules ! write_leb128 { ($ this_fn : ident , $ int_ty : ty , $ write_leb_fn : ident) => { # [inline] fn $ this_fn (& mut self , v : $ int_ty) { self . write_with (| buf | leb128 ::$ write_leb_fn (buf , v)) } } ; }
};
}
