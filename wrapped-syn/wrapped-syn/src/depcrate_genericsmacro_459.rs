// Generated macro for macro_459 (macro)
macro_rules! Depcrate_genericsmacro_459 {
() => {
// Module: crate::generics
// Provides: {"macro_459"}
// Dependencies: {}
ast_struct ! { # [doc = " Precise capturing bound: the 'use&lt;&hellip;&gt;' in `impl Trait +"] # [doc = " use<'a, T>`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PreciseCapture # full { pub use_token : Token ! [use] , pub lt_token : Token ! [<] , pub params : Punctuated < CapturedParam , Token ! [,] >, pub gt_token : Token ! [>] , } }
};
}
