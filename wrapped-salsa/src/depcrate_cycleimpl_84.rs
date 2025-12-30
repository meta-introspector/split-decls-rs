// Generated macro for impl_84 (impl)
macro_rules! Depcrate_cycleimpl_84 {
() => {
// Module: crate::cycle
// Provides: {"impl_84"}
// Dependencies: {}
impl IterationCount { pub (crate) const fn initial () -> Self { Self (0) } pub (crate) const fn is_initial (self) -> bool { self . 0 == 0 } pub (crate) const fn increment (self) -> Option < Self > { let next = Self (self . 0 + 1) ; if next . 0 <= MAX_ITERATIONS . 0 { Some (next) } else { None } } pub (crate) const fn as_u32 (self) -> u32 { self . 0 as u32 } }
};
}
