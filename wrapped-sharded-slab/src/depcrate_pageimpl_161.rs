// Generated macro for impl_161 (impl)
macro_rules! Depcrate_pageimpl_161 {
() => {
// Module: crate::page
// Provides: {"impl_161"}
// Dependencies: {}
impl < C : cfg :: Config > fmt :: Debug for Addr < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Addr") . field ("addr" , & format_args ! ("{:#0x}" , & self . addr)) . field ("index" , & self . index ()) . field ("offset" , & self . offset ()) . finish () } }
};
}
