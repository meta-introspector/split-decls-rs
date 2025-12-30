// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_x86_win64compute_abi_info {
() => {
// Module: crate::callconv::x86_win64
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < Ty > (cx : & impl HasTargetSpec , fn_abi : & mut FnAbi < '_ , Ty >) { let fixup = | a : & mut ArgAbi < '_ , Ty > , is_ret : bool | { match a . layout . backend_repr { BackendRepr :: Memory { sized : false } => { } BackendRepr :: ScalarPair (..) | BackendRepr :: Memory { sized : true } => { match a . layout . size . bits () { 8 => a . cast_to (Reg :: i8 ()) , 16 => a . cast_to (Reg :: i16 ()) , 32 => a . cast_to (Reg :: i32 ()) , 64 => a . cast_to (Reg :: i64 ()) , _ => a . make_indirect () , } } BackendRepr :: SimdVector { .. } => { } BackendRepr :: Scalar (scalar) => { if is_ret && matches ! (scalar . primitive () , Primitive :: Int (Integer :: I128 , _)) { if cx . target_spec () . rustc_abi == Some (RustcAbi :: X86Softfloat) { } else { let reg = Reg { kind : RegKind :: Vector , size : Size :: from_bits (128) } ; a . cast_to (reg) ; } } else if a . layout . size . bytes () > 8 && ! matches ! (scalar . primitive () , Primitive :: Float (Float :: F128)) { a . make_indirect () ; } else { a . extend_integer_width_to (32) ; } } } } ; if ! fn_abi . ret . is_ignore () { fixup (& mut fn_abi . ret , true) ; } for arg in fn_abi . args . iter_mut () { if arg . is_ignore () && arg . layout . is_zst () { arg . make_indirect_from_ignore () ; continue ; } fixup (arg , false) ; } }
};
}
