// Generated macro for impl_154 (impl)
macro_rules! Depcrate_pageimpl_154 {
() => {
// Module: crate::page
// Provides: {"impl_154"}
// Dependencies: {}
impl Local { pub (crate) fn new () -> Self { Self { head : UnsafeCell :: new (0) , } } # [inline (always)] fn head (& self) -> usize { self . head . with (| head | unsafe { * head }) } # [inline (always)] fn set_head (& self , new_head : usize) { self . head . with_mut (| head | unsafe { * head = new_head ; }) } }
};
}
