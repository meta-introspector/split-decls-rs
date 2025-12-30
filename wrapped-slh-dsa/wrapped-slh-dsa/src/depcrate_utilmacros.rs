// Generated macro for macros (module)
macro_rules! Depcrate_utilmacros {
() => {
// Module: crate::util
// Provides: {"macros"}
// Dependencies: {}
# [cfg (test)] pub (crate) mod macros { # [doc = " Generate a test case"] # [macro_export] macro_rules ! gen_test { ($ name : ident , $ t : ty) => { paste :: paste ! { # [test] fn [<$ name _ $ t : lower >] () { $ name ::<$ t > () } } } ; } macro_rules ! test_parameter_sets { ($ name : ident) => { # [allow (unused_imports)] use crate :: hashes ::*; crate :: gen_test ! ($ name , Shake128f) ; crate :: gen_test ! ($ name , Shake128s) ; crate :: gen_test ! ($ name , Shake192f) ; crate :: gen_test ! ($ name , Shake192s) ; crate :: gen_test ! ($ name , Shake256f) ; crate :: gen_test ! ($ name , Shake256s) ; crate :: gen_test ! ($ name , Sha2_128f) ; crate :: gen_test ! ($ name , Sha2_128s) ; crate :: gen_test ! ($ name , Sha2_192f) ; crate :: gen_test ! ($ name , Sha2_192s) ; crate :: gen_test ! ($ name , Sha2_256f) ; crate :: gen_test ! ($ name , Sha2_256s) ; } ; } pub (crate) use test_parameter_sets ; }
};
}
