// Generated macro for impl_47 (impl)
macro_rules! Depcrate_serializeimpl_47 {
() => {
// Module: crate::serialize
// Provides: {"impl_47"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for Cow < '_ , str > { fn decode (d : & mut D) -> Cow < 'static , str > { let v : String = Decodable :: decode (d) ; Cow :: Owned (v) } }
};
}
