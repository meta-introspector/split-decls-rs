// Generated macro for impl_433 (impl)
macro_rules! Depcrateimpl_433 {
() => {
// Module: crate
// Provides: {"impl_433"}
// Dependencies: {}
impl < D : rustc_serialize :: Decoder > Decodable < D > for ErrorGuaranteed { # [inline] fn decode (_d : & mut D) -> ErrorGuaranteed { panic ! ("`ErrorGuaranteed` should never have been serialized to metadata or incremental caches") } }
};
}
