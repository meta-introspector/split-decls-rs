// Generated macro for impl_1009 (impl)
macro_rules! Depcrate_overflowimpl_1009 {
() => {
// Module: crate::overflow
// Provides: {"impl_1009"}
// Dependencies: {}
impl < 'a , T : 'a + IntoOverflowableItem < 'a > > IntoOverflowableItem < 'a > for ptr :: P < T > { fn into_overflowable_item (& 'a self) -> OverflowableItem < 'a > { (* * self) . into_overflowable_item () } }
};
}
