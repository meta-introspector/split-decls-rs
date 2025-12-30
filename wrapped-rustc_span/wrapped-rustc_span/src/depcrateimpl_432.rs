// Generated macro for impl_432 (impl)
macro_rules! Depcrateimpl_432 {
() => {
// Module: crate
// Provides: {"impl_432"}
// Dependencies: {}
impl < E : rustc_serialize :: Encoder > Encodable < E > for ErrorGuaranteed { # [inline] fn encode (& self , _e : & mut E) { panic ! ("should never serialize an `ErrorGuaranteed`, as we do not write metadata or \
            incremental caches in case errors occurred") } }
};
}
