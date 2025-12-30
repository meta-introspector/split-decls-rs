// Generated macro for impl_388 (impl)
macro_rules! Depcrate_util_linked_listimpl_388 {
() => {
// Module: crate::util::linked_list
// Provides: {"impl_388"}
// Dependencies: {}
impl < L : Link > fmt :: Debug for LinkedList < L , L :: Target > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("LinkedList") . field ("head" , & self . head) . field ("tail" , & self . tail) . finish () } }
};
}
