// Generated macro for classify_ret_ty (function)
macro_rules! Depcrate_callconv_xtensaclassify_ret_ty {
() => {
// Module: crate::callconv::xtensa
// Provides: {"classify_ret_ty"}
// Dependencies: {}
fn classify_ret_ty < 'a , Ty , C > (arg : & mut ArgAbi < '_ , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , { if arg . is_ignore () { return ; } let mut arg_gprs_left = NUM_RET_GPRS ; classify_arg_ty (arg , & mut arg_gprs_left , MAX_RET_IN_REGS_SIZE) ; match arg . mode { super :: PassMode :: Indirect { attrs : _ , meta_attrs : _ , ref mut on_stack } => { * on_stack = false ; } _ => { } } }
};
}
