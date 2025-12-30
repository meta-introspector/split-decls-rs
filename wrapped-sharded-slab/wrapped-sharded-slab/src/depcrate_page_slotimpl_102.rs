// Generated macro for impl_102 (impl)
macro_rules! Depcrate_page_slotimpl_102 {
() => {
// Module: crate::page::slot
// Provides: {"impl_102"}
// Dependencies: {}
impl < C : cfg :: Config > Pack < C > for Generation < C > { # [doc = " Use all the remaining bits in the word for the generation counter, minus"] # [doc = " any bits reserved by the user."] const LEN : usize = (cfg :: WIDTH - C :: RESERVED_BITS) - Self :: SHIFT ; type Prev = Tid < C > ; # [inline (always)] fn from_usize (u : usize) -> Self { debug_assert ! (u <= Self :: BITS) ; Self :: new (u) } # [inline (always)] fn as_usize (& self) -> usize { self . value } }
};
}
