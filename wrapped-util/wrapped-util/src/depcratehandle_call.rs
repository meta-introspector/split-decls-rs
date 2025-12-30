// Generated macro for handle_call (macro)
macro_rules! Depcratehandle_call {
() => {
// Module: crate
// Provides: {"handle_call"}
// Dependencies: {}
macro_rules ! handle_call { (fn_name : $ fn_name : ident , CFn : $ CFn : ty , RustFn : $ RustFn : ty , RustArgs : $ RustArgs : ty , attrs : [$ ($ attr : meta) ,*] , extra : ($ basis : ident , $ op : ident , $ inputs : ident) , fn_extra : $ musl_fn : expr ,) => { $ (# [$ attr]) * if $ op == stringify ! ($ fn_name) { type Op = libm_test :: op ::$ fn_name :: Routine ; let input = <$ RustArgs >:: parse ($ inputs) ; let libm_fn : < Op as MathOp >:: RustFn = libm ::$ fn_name ; let output = match $ basis { "libm" => input . call_intercept_panics (libm_fn) , # [cfg (feature = "build-musl")] "musl" => { let musl_fn : < Op as MathOp >:: CFn = $ musl_fn . unwrap_or_else (|| panic ! ("no musl function for {}" , $ op)) ; input . call (musl_fn) } # [cfg (feature = "build-mpfr")] "mpfr" => { let mut mp = < Op as MpOp >:: new_mp () ; Op :: run (& mut mp , input) } _ => panic ! ("unrecognized or disabled basis '{}'" , $ basis) , } ; println ! ("{output:?} {:x}" , Hexf (output)) ; return ; } } ; }
};
}
