// Generated macro for impl_155 (impl)
macro_rules! Depcrate_pageimpl_155 {
() => {
// Module: crate::page
// Provides: {"impl_155"}
// Dependencies: {}
impl < C : cfg :: Config > FreeList < C > for Local { fn push < T > (& self , new_head : usize , slot : & Slot < T , C >) { slot . set_next (self . head ()) ; self . set_head (new_head) ; } }
};
}
