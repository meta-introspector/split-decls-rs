// Generated macro for impl_119 (impl)
macro_rules! Depcrate_page_slotimpl_119 {
() => {
// Module: crate::page::slot
// Provides: {"impl_119"}
// Dependencies: {}
impl < C : cfg :: Config > Pack < C > for Lifecycle < C > { const LEN : usize = 2 ; type Prev = () ; fn from_usize (u : usize) -> Self { Self { state : match u & Self :: MASK { 0b00 => State :: Present , 0b01 => State :: Marked , 0b11 => State :: Removing , bad => unreachable ! ("weird lifecycle {:#b}" , bad) , } , _cfg : PhantomData , } } fn as_usize (& self) -> usize { self . state as usize } }
};
}
