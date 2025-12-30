// Generated macro for impl_45 (impl)
macro_rules! Depcrate_serializeimpl_45 {
() => {
// Module: crate::serialize
// Provides: {"impl_45"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > + ToOwned > Decodable < D > for Cow < 'static , [T] > where [T] : ToOwned < Owned = Vec < T > > , { fn decode (d : & mut D) -> Cow < 'static , [T] > { let v : Vec < T > = Decodable :: decode (d) ; Cow :: Owned (v) } }
};
}
