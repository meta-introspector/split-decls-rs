// Generated macro for impl_110 (impl)
macro_rules! Depcrate_page_slotimpl_110 {
() => {
// Module: crate::page::slot
// Provides: {"impl_110"}
// Dependencies: {}
impl < C : cfg :: Config > Generation < C > { fn advance (self) -> Self { Self :: from_usize ((self . value + 1) % Self :: BITS) } }
};
}
