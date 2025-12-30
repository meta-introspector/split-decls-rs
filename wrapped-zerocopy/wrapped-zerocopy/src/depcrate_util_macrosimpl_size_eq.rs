// Generated macro for impl_size_eq (macro)
macro_rules! Depcrate_util_macrosimpl_size_eq {
() => {
// Module: crate::util::macros
// Provides: {"impl_size_eq"}
// Dependencies: {}
# [rustfmt :: skip] macro_rules ! impl_size_eq { ($ t : ty , $ u : ty) => { const _ : () = { use crate :: { KnownLayout , pointer :: { PtrInner , SizeEq } } ; static_assert ! (=> { let t = <$ t as KnownLayout >:: LAYOUT ; let u = <$ u as KnownLayout >:: LAYOUT ; t . align . get () >= u . align . get () && match (t . size_info , u . size_info) { (SizeInfo :: Sized { size : t } , SizeInfo :: Sized { size : u }) => t == u , (SizeInfo :: SliceDst (TrailingSliceLayout { offset : t_offset , elem_size : t_elem_size }) , SizeInfo :: SliceDst (TrailingSliceLayout { offset : u_offset , elem_size : u_elem_size })) => t_offset == u_offset && t_elem_size == u_elem_size , _ => false , } }) ; unsafe impl SizeEq <$ t > for $ u { # [inline (always)] fn cast_from_raw (t : PtrInner <'_ , $ t >) -> PtrInner <'_ , $ u > { unsafe { cast ! (t) } } } unsafe impl SizeEq <$ u > for $ t { # [inline (always)] fn cast_from_raw (u : PtrInner <'_ , $ u >) -> PtrInner <'_ , $ t > { unsafe { cast ! (u) } } } } ; } ; }
};
}
