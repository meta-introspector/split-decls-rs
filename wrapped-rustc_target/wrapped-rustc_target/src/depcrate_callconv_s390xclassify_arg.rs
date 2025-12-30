// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_s390xclassify_arg {
() => {
// Module: crate::callconv::s390x
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < 'a , Ty , C > (cx : & C , arg : & mut ArgAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { if ! arg . layout . is_sized () { return ; } if arg . is_ignore () { if cx . target_spec () . os == "linux" && matches ! (&* cx . target_spec () . env , "gnu" | "musl" | "uclibc") && arg . layout . is_zst () { arg . make_indirect_from_ignore () ; } return ; } let size = arg . layout . size ; if size . bits () <= 128 { if let BackendRepr :: SimdVector { .. } = arg . layout . backend_repr { return ; } if arg . layout . is_single_vector_element (cx , size) { arg . cast_to (Reg { kind : RegKind :: Vector , size }) ; return ; } } if ! arg . layout . is_aggregate () && size . bits () <= 64 { arg . extend_integer_width_to (64) ; return ; } if arg . layout . is_single_fp_element (cx) { match size . bytes () { 4 => arg . cast_to (Reg :: f32 ()) , 8 => arg . cast_to (Reg :: f64 ()) , _ => arg . make_indirect () , } } else { match size . bytes () { 1 => arg . cast_to (Reg :: i8 ()) , 2 => arg . cast_to (Reg :: i16 ()) , 4 => arg . cast_to (Reg :: i32 ()) , 8 => arg . cast_to (Reg :: i64 ()) , _ => arg . make_indirect () , } } }
};
}
