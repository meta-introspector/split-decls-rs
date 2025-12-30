// Generated macro for impl_52 (impl)
macro_rules! Depcrate_parts_write_adapterimpl_52 {
() => {
// Module: crate::parts_write_adapter
// Provides: {"impl_52"}
// Dependencies: {}
impl < W : fmt :: Write + ? Sized > fmt :: Write for CoreWriteAsPartsWrite < W > { # [inline] fn write_str (& mut self , s : & str) -> fmt :: Result { self . 0 . write_str (s) } # [inline] fn write_char (& mut self , c : char) -> fmt :: Result { self . 0 . write_char (c) } }
};
}
