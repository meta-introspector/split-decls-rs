// Generated macro for msvc_args (macro)
macro_rules! Depcrate_compiler_msvcmsvc_args {
() => {
// Module: crate::compiler::msvc
// Provides: {"msvc_args"}
// Dependencies: {}
macro_rules ! msvc_args { (static ARGS : [$ t : ty ; _] = [$ ($ macro : ident ! ($ ($ v : tt) *) ,) *]) => { counted_array ! (static ARGS : [$ t ; _] = [$ (msvc_args ! (@ one "-" , $ macro ! ($ ($ v) *)) ,) *]) ; counted_array ! (static SLASH_ARGS : [$ t ; _] = [$ (msvc_args ! (@ one "/" , $ macro ! ($ ($ v) *)) ,) *]) ; } ; (@ one $ prefix : expr , msvc_take_arg ! ($ s : expr , $ ($ t : tt) *)) => { take_arg ! (concat ! ($ prefix , $ s) , $ ($ t) +) } ; (@ one $ prefix : expr , msvc_flag ! ($ s : expr , $ ($ t : tt) +)) => { flag ! (concat ! ($ prefix , $ s) , $ ($ t) +) } ; (@ one $ prefix : expr , $ other : expr) => { $ other } ; }
};
}
