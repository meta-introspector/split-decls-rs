// Generated macro for impl_153 (impl)
macro_rules! Depcrate_queueimpl_153 {
() => {
// Module: crate::queue
// Provides: {"impl_153"}
// Dependencies: {}
impl < T : Debug > Debug for Queue < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_set () ; let guard = Guard :: new () ; let mut current = self . oldest . load (Acquire , & guard) ; while let Some (entry) = current . as_ref () { let next = entry . next_ptr (Acquire , & guard) ; d . entry (entry) ; current = next ; } d . finish () } }
};
}
