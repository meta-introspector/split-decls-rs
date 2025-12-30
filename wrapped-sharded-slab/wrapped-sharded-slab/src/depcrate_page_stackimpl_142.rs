// Generated macro for impl_142 (impl)
macro_rules! Depcrate_page_stackimpl_142 {
() => {
// Module: crate::page::stack
// Provides: {"impl_142"}
// Dependencies: {}
impl < C > fmt :: Debug for TransferStack < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TransferStack") . field ("head" , & format_args ! ("{:#0x}" , & self . head . load (Ordering :: Relaxed)) ,) . finish () } }
};
}
