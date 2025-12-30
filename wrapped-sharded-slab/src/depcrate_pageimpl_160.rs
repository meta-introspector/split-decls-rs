// Generated macro for impl_160 (impl)
macro_rules! Depcrate_pageimpl_160 {
() => {
// Module: crate::page
// Provides: {"impl_160"}
// Dependencies: {}
impl < C , T > fmt :: Debug for Shared < C , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Shared") . field ("remote" , & self . remote) . field ("prev_sz" , & self . prev_sz) . field ("size" , & self . size) . finish () } }
};
}
