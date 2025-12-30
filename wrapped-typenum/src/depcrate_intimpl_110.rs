// Generated macro for impl_110 (impl)
macro_rules! Depcrate_intimpl_110 {
() => {
// Module: crate::int
// Provides: {"impl_110"}
// Dependencies: {}
# [doc = " X > - Y"] impl < P : Unsigned + NonZero , N : Unsigned + NonZero > Cmp < NInt < N > > for PInt < P > { type Output = Greater ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & NInt < N >) -> Self :: Output { Greater } }
};
}
