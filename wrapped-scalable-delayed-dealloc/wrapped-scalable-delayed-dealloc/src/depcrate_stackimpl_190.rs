// Generated macro for impl_190 (impl)
macro_rules! Depcrate_stackimpl_190 {
() => {
// Module: crate::stack
// Provides: {"impl_190"}
// Dependencies: {}
impl < T : Debug > Debug for Stack < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_set () ; let guard = Guard :: new () ; let mut current = self . newest . load (Acquire , & guard) ; while let Some (entry) = current . as_ref () { let next = entry . next_ptr (Acquire , & guard) ; d . entry (entry) ; current = next ; } d . finish () } }
};
}
