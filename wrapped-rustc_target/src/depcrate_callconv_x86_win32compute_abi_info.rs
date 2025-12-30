// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_x86_win32compute_abi_info {
() => {
// Module: crate::callconv::x86_win32
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty > , opts : super :: x86 :: X86Options ,) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { if ! fn_abi . ret . is_ignore () { if fn_abi . ret . layout . is_aggregate () && fn_abi . ret . layout . is_sized () { let t = cx . target_spec () ; if t . abi_return_struct_as_int || opts . reg_struct_return { match fn_abi . ret . layout . size . bytes () { 1 => fn_abi . ret . cast_to (Reg :: i8 ()) , 2 => fn_abi . ret . cast_to (Reg :: i16 ()) , 4 => fn_abi . ret . cast_to (Reg :: i32 ()) , 8 => fn_abi . ret . cast_to (Reg :: i64 ()) , _ => fn_abi . ret . make_indirect () , } } else { fn_abi . ret . make_indirect () ; } } else { fn_abi . ret . extend_integer_width_to (32) ; } } for arg in fn_abi . args . iter_mut () { if arg . is_ignore () || ! arg . layout . is_sized () { continue ; } let align_4 = Align :: from_bytes (4) . unwrap () ; if arg . layout . is_adt () && let Some (max_repr_align) = arg . layout . max_repr_align && max_repr_align > align_4 { assert ! (arg . layout . align . abi >= max_repr_align , "abi alignment {:?} less than requested alignment {max_repr_align:?}" , arg . layout . align . abi ,) ; arg . make_indirect () ; } else if arg . layout . is_aggregate () { let byval_align = align_4 ; arg . pass_by_stack_offset (Some (byval_align)) ; } else { arg . extend_integer_width_to (32) ; } } super :: x86 :: fill_inregs (cx , fn_abi , opts , false) ; }
};
}
