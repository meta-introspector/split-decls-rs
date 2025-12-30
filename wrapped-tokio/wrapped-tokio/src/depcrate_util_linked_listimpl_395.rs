// Generated macro for impl_395 (impl)
macro_rules! Depcrate_util_linked_listimpl_395 {
() => {
// Module: crate::util::linked_list
// Provides: {"impl_395"}
// Dependencies: {}
impl < T > fmt :: Debug for Pointers < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let prev = self . get_prev () ; let next = self . get_next () ; f . debug_struct ("Pointers") . field ("prev" , & prev) . field ("next" , & next) . finish () } }
};
}
