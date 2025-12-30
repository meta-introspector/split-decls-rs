// Generated macro for impl_152 (impl)
macro_rules! Depcrate_queueimpl_152 {
() => {
// Module: crate::queue
// Provides: {"impl_152"}
// Dependencies: {}
impl < T : Clone > Clone for Queue < T > { # [inline] fn clone (& self) -> Self { let self_clone = Self :: default () ; let guard = Guard :: new () ; let mut current = self . oldest . load (Acquire , & guard) ; while let Some (entry) = current . as_ref () { let next = entry . next_ptr (Acquire , & guard) ; let _result = self_clone . push_if_internal ((* * entry) . clone () , | _ | true , & guard) ; current = next ; } self_clone } }
};
}
