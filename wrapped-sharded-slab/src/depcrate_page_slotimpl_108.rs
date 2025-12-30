// Generated macro for impl_108 (impl)
macro_rules! Depcrate_page_slotimpl_108 {
() => {
// Module: crate::page::slot
// Provides: {"impl_108"}
// Dependencies: {}
impl < T , C : cfg :: Config > fmt :: Debug for Slot < T , C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let lifecycle = self . lifecycle . load (Ordering :: Relaxed) ; f . debug_struct ("Slot") . field ("lifecycle" , & format_args ! ("{:#x}" , lifecycle)) . field ("state" , & Lifecycle :: < C > :: from_packed (lifecycle) . state) . field ("gen" , & LifecycleGen :: < C > :: from_packed (lifecycle) . 0) . field ("refs" , & RefCount :: < C > :: from_packed (lifecycle)) . field ("next" , & self . next ()) . finish () } }
};
}
