// Generated macro for impl_449 (impl)
macro_rules! Depcrateimpl_449 {
() => {
// Module: crate
// Provides: {"impl_449"}
// Dependencies: {}
impl < D : rustc_serialize :: Decoder > Decodable < D > for ErrorGuaranteed { # [inline] fn decode (_d : & mut D) -> ErrorGuaranteed { panic ! ("`ErrorGuaranteed` should never have been serialized to metadata or incremental caches") } }
};
}
