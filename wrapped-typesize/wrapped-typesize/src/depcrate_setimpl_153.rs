// Generated macro for impl_153 (impl)
macro_rules! Depcrate_setimpl_153 {
() => {
// Module: crate::set
// Provides: {"impl_153"}
// Dependencies: {}
impl < T : TypeSize , S > TypeSize for HashSet < T , S > { fn extra_size (& self) -> usize { generic_vec_extra_size :: < T > (self . iter () , self . capacity () , self . len ()) } }
};
}
