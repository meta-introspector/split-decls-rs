// Generated macro for impl_149 (impl)
macro_rules! Depcrate_pageimpl_149 {
() => {
// Module: crate::page
// Provides: {"impl_149"}
// Dependencies: {}
impl < C : cfg :: Config > Pack < C > for Addr < C > { const LEN : usize = C :: MAX_PAGES + C :: ADDR_INDEX_SHIFT ; type Prev = () ; fn as_usize (& self) -> usize { self . addr } fn from_usize (addr : usize) -> Self { debug_assert ! (addr <= Self :: BITS) ; Self { addr , _cfg : PhantomData , } } }
};
}
