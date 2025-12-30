// Generated macro for impl_66 (impl)
macro_rules! Depcrate_serializeimpl_66 {
() => {
// Module: crate::serialize
// Provides: {"impl_66"}
// Dependencies: {}
impl < S : Encoder , A : Array < Item : Encodable < S > > > Encodable < S > for SmallVec < A > { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
};
}
