// Generated macro for impl_106 (impl)
macro_rules! Depcrate_intimpl_106 {
() => {
// Module: crate::int
// Provides: {"impl_106"}
// Dependencies: {}
# [doc = " -X < Y"] impl < P : Unsigned + NonZero , N : Unsigned + NonZero > Cmp < PInt < P > > for NInt < N > { type Output = Less ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & PInt < P >) -> Self :: Output { Less } }
};
}
