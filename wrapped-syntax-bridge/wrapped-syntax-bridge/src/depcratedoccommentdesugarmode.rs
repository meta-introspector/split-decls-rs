// Generated macro for DocCommentDesugarMode (enum)
macro_rules! DepcrateDocCommentDesugarMode {
() => {
// Module: crate
// Provides: {"DocCommentDesugarMode"}
// Dependencies: {}
# [doc = " Doc comment desugaring differs between mbe and proc-macros."] # [derive (Copy , Clone , PartialEq , Eq)] pub enum DocCommentDesugarMode { # [doc = " Desugars doc comments as quoted raw strings"] Mbe , # [doc = " Desugars doc comments as quoted strings"] ProcMacro , }
};
}
