// Generated macro for impl_107 (impl)
macro_rules! Depcrate_intimpl_107 {
() => {
// Module: crate::int
// Provides: {"impl_107"}
// Dependencies: {}
# [doc = " X > - Y"] impl < P : Unsigned + NonZero , N : Unsigned + NonZero > Cmp < NInt < N > > for PInt < P > { type Output = Greater ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & NInt < N >) -> Self :: Output { Greater } }
};
}
