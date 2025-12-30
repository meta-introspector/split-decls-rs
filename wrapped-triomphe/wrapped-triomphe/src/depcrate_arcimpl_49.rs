// Generated macro for impl_49 (impl)
macro_rules! Depcrate_arcimpl_49 {
() => {
// Module: crate::arc
// Provides: {"impl_49"}
// Dependencies: {}
impl < T : ? Sized + PartialEq > PartialEq for Arc < T > { fn eq (& self , other : & Arc < T >) -> bool { Self :: ptr_eq (self , other) || * (* self) == * (* other) } # [allow (clippy :: partialeq_ne_impl)] fn ne (& self , other : & Arc < T >) -> bool { ! Self :: ptr_eq (self , other) && * (* self) != * (* other) } }
};
}
