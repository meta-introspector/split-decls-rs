// Generated macro for impl_451 (impl)
macro_rules! Depcrate_collections_hash_setimpl_451 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_451"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , S > Clone for HashSet < T , S > where T : Clone , S : Clone , { # [inline] fn clone (& self) -> Self { Self { base : self . base . clone () } } # [doc = " Overwrites the contents of `self` with a clone of the contents of `source`."] # [doc = ""] # [doc = " This method is preferred over simply assigning `source.clone()` to `self`,"] # [doc = " as it avoids reallocation if possible."] # [inline] fn clone_from (& mut self , other : & Self) { self . base . clone_from (& other . base) ; } }
};
}
