// Generated macro for impl_99 (impl)
macro_rules! Depcrate_linked_listimpl_99 {
() => {
// Module: crate::linked_list
// Provides: {"impl_99"}
// Dependencies: {}
impl < T : Debug > Debug for LinkedEntry < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Entry") . field ("instance" , & self . instance) . field ("next" , & self . next) . field ("removed" , & self . is_deleted (Relaxed)) . finish () } }
};
}
