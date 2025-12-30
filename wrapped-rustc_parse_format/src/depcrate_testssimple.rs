// Generated macro for simple (function)
macro_rules! Depcrate_testssimple {
() => {
// Module: crate::tests
// Provides: {"simple"}
// Dependencies: {}
# [test] fn simple () { same ("asdf" , & [Lit ("asdf")]) ; same ("a{{b" , & [Lit ("a") , Lit ("{b")]) ; same ("a}}b" , & [Lit ("a") , Lit ("}b")]) ; same ("a}}" , & [Lit ("a") , Lit ("}")]) ; same ("}}" , & [Lit ("}")]) ; same ("\\}}" , & [Lit ("\\") , Lit ("}")]) ; }
};
}
