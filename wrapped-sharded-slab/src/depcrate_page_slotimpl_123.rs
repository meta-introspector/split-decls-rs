// Generated macro for impl_123 (impl)
macro_rules! Depcrate_page_slotimpl_123 {
() => {
// Module: crate::page::slot
// Provides: {"impl_123"}
// Dependencies: {}
impl < C : cfg :: Config > Pack < C > for RefCount < C > { const LEN : usize = cfg :: WIDTH - (Lifecycle :: < C > :: LEN + Generation :: < C > :: LEN) ; type Prev = Lifecycle < C > ; fn from_usize (value : usize) -> Self { debug_assert ! (value <= Self :: BITS) ; Self { value , _cfg : PhantomData , } } fn as_usize (& self) -> usize { self . value } }
};
}
