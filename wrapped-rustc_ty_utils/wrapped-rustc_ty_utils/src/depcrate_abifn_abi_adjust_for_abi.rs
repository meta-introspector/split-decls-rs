// Generated macro for fn_abi_adjust_for_abi (function)
macro_rules! Depcrate_abifn_abi_adjust_for_abi {
() => {
// Module: crate::abi
// Provides: {"fn_abi_adjust_for_abi"}
// Dependencies: {}
# [tracing :: instrument (level = "trace" , skip (cx))] fn fn_abi_adjust_for_abi < 'tcx > (cx : & LayoutCx < 'tcx > , fn_abi : & mut FnAbi < 'tcx , Ty < 'tcx > > , abi : ExternAbi , fn_def_id : Option < DefId > ,) { if abi == ExternAbi :: Unadjusted { fn unadjust < 'tcx > (arg : & mut ArgAbi < 'tcx , Ty < 'tcx > >) { if matches ! (arg . layout . backend_repr , BackendRepr :: Memory { .. }) { assert ! (arg . layout . backend_repr . is_sized () , "'unadjusted' ABI does not support unsized arguments") ; } arg . make_direct_deprecated () ; } unadjust (& mut fn_abi . ret) ; for arg in fn_abi . args . iter_mut () { unadjust (arg) ; } return ; } let tcx = cx . tcx () ; if abi . is_rustic_abi () { fn_abi . adjust_for_rust_abi (cx) ; let deduced_param_attrs = if tcx . sess . opts . optimize != OptLevel :: No && tcx . sess . opts . incremental . is_none () { fn_def_id . map (| fn_def_id | tcx . deduced_param_attrs (fn_def_id)) . unwrap_or_default () } else { & [] } ; for (arg_idx , arg) in fn_abi . args . iter_mut () . enumerate () { if arg . is_ignore () { continue ; } if let & mut PassMode :: Indirect { ref mut attrs , .. } = & mut arg . mode { if let Some (deduced_param_attrs) = deduced_param_attrs . get (arg_idx) && deduced_param_attrs . read_only { attrs . regular . insert (ArgAttribute :: ReadOnly) ; debug ! ("added deduced read-only attribute") ; } } } } else { fn_abi . adjust_for_foreign_abi (cx , abi) ; } }
};
}
