// Generated macro for impl_141 (impl)
macro_rules! Depcrate_page_stackimpl_141 {
() => {
// Module: crate::page::stack
// Provides: {"impl_141"}
// Dependencies: {}
impl < C : cfg :: Config > super :: FreeList < C > for TransferStack < C > { fn push < T > (& self , new_head : usize , slot : & super :: Slot < T , C >) { self . push (new_head , | next | slot . set_next (next)) } }
};
}
