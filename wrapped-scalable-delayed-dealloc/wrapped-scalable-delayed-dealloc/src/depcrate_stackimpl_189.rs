// Generated macro for impl_189 (impl)
macro_rules! Depcrate_stackimpl_189 {
() => {
// Module: crate::stack
// Provides: {"impl_189"}
// Dependencies: {}
impl < T : Clone > Clone for Stack < T > { # [inline] fn clone (& self) -> Self { let self_clone = Self :: default () ; let guard = Guard :: new () ; let mut current = self . newest . load (Acquire , & guard) ; let mut oldest : Option < Shared < LinkedEntry < T > > > = None ; while let Some (entry) = current . as_ref () { let new_entry = unsafe { Shared :: new_unchecked (LinkedEntry :: new ((* * entry) . clone ())) } ; if let Some (oldest) = oldest . take () { oldest . next () . swap ((Some (new_entry . clone ()) , Tag :: None) , Acquire) ; } else { self_clone . newest . swap ((Some (new_entry . clone ()) , Tag :: None) , Acquire) ; } oldest . replace (new_entry) ; current = entry . next_ptr (Acquire , & guard) ; } self_clone } }
};
}
