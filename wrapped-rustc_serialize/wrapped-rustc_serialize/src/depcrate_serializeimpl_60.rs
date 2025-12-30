// Generated macro for impl_60 (impl)
macro_rules! Depcrate_serializeimpl_60 {
() => {
// Module: crate::serialize
// Provides: {"impl_60"}
// Dependencies: {}
impl < S : Encoder , T : Encodable < S > > Encodable < S > for RefCell < T > { fn encode (& self , s : & mut S) { self . borrow () . encode (s) ; } }
};
}
