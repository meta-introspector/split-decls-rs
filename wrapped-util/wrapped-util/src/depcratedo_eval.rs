// Generated macro for do_eval (function)
macro_rules! Depcratedo_eval {
() => {
// Module: crate
// Provides: {"do_eval"}
// Dependencies: {}
# [doc = " Evaluate the specified operation with a given basis."] fn do_eval (basis : & str , op : & str , inputs : & [& str]) { libm_macros :: for_each_function ! { callback : handle_call , emit_types : [CFn , RustFn , RustArgs] , extra : (basis , op , inputs) , fn_extra : match MACRO_FN_NAME { fmaximum | fmaximum_num | fmaximum_numf | fmaximumf | fminimum | fminimum_num | fminimum_numf | fminimumf | roundeven | roundevenf | ALL_F16 | ALL_F128 => None , _ => Some (musl_math_sys :: MACRO_FN_NAME) } } panic ! ("no operation matching {op}") ; }
};
}
