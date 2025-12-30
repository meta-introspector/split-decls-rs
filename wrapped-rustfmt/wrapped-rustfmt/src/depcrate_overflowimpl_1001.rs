// Generated macro for impl_1001 (impl)
macro_rules! Depcrate_overflowimpl_1001 {
() => {
// Module: crate::overflow
// Provides: {"impl_1001"}
// Dependencies: {}
impl < 'a , T : 'a + IntoOverflowableItem < 'a > > IntoOverflowableItem < 'a > for Box < T > { fn into_overflowable_item (& 'a self) -> OverflowableItem < 'a > { (* * self) . into_overflowable_item () } }
};
}
