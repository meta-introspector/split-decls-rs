// Generated macro for array (function)
macro_rules! Depcrate_utilarray {
() => {
// Module: crate::util
// Provides: {"array"}
// Dependencies: {}
pub (crate) fn array (base_ty : & str , pos : TypePosition , immutable : bool) -> syn :: Type { match pos { TypePosition :: Argument => { shared_ref (slice_ty (ident_ty (raw_ident (base_ty))) , ! immutable ,) } TypePosition :: Return => vec_ty (ident_ty (raw_ident (base_ty))) , } }
};
}
