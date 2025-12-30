// Generated macro for impl_44 (impl)
macro_rules! Depcrate_serializeimpl_44 {
() => {
// Module: crate::serialize
// Provides: {"impl_44"}
// Dependencies: {}
impl < S : Encoder , T : Encodable < S > > Encodable < S > for Cow < '_ , [T] > where [T] : ToOwned < Owned = Vec < T > > , { fn encode (& self , s : & mut S) { let slice : & [T] = self ; slice . encode (s) ; } }
};
}
