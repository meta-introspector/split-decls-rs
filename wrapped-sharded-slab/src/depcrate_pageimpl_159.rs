// Generated macro for impl_159 (impl)
macro_rules! Depcrate_pageimpl_159 {
() => {
// Module: crate::page
// Provides: {"impl_159"}
// Dependencies: {}
impl fmt :: Debug for Local { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . head . with (| head | { let head = unsafe { * head } ; f . debug_struct ("Local") . field ("head" , & format_args ! ("{:#0x}" , head)) . finish () }) } }
};
}
