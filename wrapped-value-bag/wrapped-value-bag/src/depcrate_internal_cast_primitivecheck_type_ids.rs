// Generated macro for check_type_ids (macro)
macro_rules! Depcrate_internal_cast_primitivecheck_type_ids {
() => {
// Module: crate::internal::cast::primitive
// Provides: {"check_type_ids"}
// Dependencies: {}
macro_rules ! check_type_ids { (&$ l : lifetime $ v : ident => $ ($ (# [cfg ($ ($ cfg : tt) *)]) * $ ty : ty ,) *) => { $ ($ (# [cfg ($ ($ cfg) *)]) * if TypeId :: of ::< T > () == TypeId :: of ::<$ ty > () { let v = unsafe { * ($ v . 0 as * const & $ l $ ty) } ; return Some (ValueBag :: from (v)) ; }) * $ ($ (# [cfg ($ ($ cfg) *)]) * if TypeId :: of ::< T > () == TypeId :: of ::< Option <$ ty >> () { let v = unsafe { * ($ v . 0 as * const & $ l Option <$ ty >) } ; if let Some (v) = v { return Some (ValueBag :: from (v)) ; } else { return Some (ValueBag :: empty ()) ; } }) * } ; }
};
}
