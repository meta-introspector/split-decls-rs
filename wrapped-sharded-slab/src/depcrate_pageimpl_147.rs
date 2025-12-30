// Generated macro for impl_147 (impl)
macro_rules! Depcrate_pageimpl_147 {
() => {
// Module: crate::page
// Provides: {"impl_147"}
// Dependencies: {}
impl < C : cfg :: Config > Addr < C > { const NULL : usize = Self :: BITS + 1 ; pub (crate) fn index (self) -> usize { let shifted = (self . addr + C :: INITIAL_SZ) >> C :: ADDR_INDEX_SHIFT ; cfg :: WIDTH - shifted . leading_zeros () as usize } pub (crate) fn offset (self) -> usize { self . addr } }
};
}
