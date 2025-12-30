// Generated macro for impl_56 (impl)
macro_rules! Depcrate_parts_write_adapterimpl_56 {
() => {
// Module: crate::parts_write_adapter
// Provides: {"impl_56"}
// Dependencies: {}
impl < T : Writeable + ? Sized > fmt :: Display for WithPart < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Writeable :: write_to (& self , f) } }
};
}
