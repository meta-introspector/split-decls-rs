// Generated macro for impl_61 (impl)
macro_rules! Depcrate_serializeimpl_61 {
() => {
// Module: crate::serialize
// Provides: {"impl_61"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > > Decodable < D > for RefCell < T > { fn decode (d : & mut D) -> RefCell < T > { RefCell :: new (Decodable :: decode (d)) } }
};
}
