// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
# [cfg (feature = "futures")] impl < S : futures_core :: Stream > SyncStream < S > { pub fn new (inner : S) -> Self { Self { inner : SyncWrapper :: new (inner) } } pub fn into_inner (self) -> S { self . inner . into_inner () } }
};
}
