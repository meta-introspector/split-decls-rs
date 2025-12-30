// Generated macro for impl_134 (impl)
macro_rules! Depcrate_internal_seqimpl_134 {
() => {
// Module: crate::internal::seq
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a , S : Extend < Option < T > > , T : for < 'b > TryFrom < ValueBag < 'b > > > ExtendValue < 'a > for ExtendPrimitive < S , T > { fn extend (& mut self , inner : Internal) { self . 0 . extend (Some (ValueBag { inner } . try_into () . ok ())) } }
};
}
