// Generated macro for impl_53 (impl)
macro_rules! Depcrate_parts_write_adapterimpl_53 {
() => {
// Module: crate::parts_write_adapter
// Provides: {"impl_53"}
// Dependencies: {}
impl < W : fmt :: Write + ? Sized > PartsWrite for CoreWriteAsPartsWrite < W > { type SubPartsWrite = CoreWriteAsPartsWrite < W > ; # [inline] fn with_part (& mut self , _part : Part , mut f : impl FnMut (& mut Self :: SubPartsWrite) -> fmt :: Result ,) -> fmt :: Result { f (self) } }
};
}
