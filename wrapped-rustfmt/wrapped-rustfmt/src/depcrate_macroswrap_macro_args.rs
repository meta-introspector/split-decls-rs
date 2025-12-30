// Generated macro for wrap_macro_args (function)
macro_rules! Depcrate_macroswrap_macro_args {
() => {
// Module: crate::macros
// Provides: {"wrap_macro_args"}
// Dependencies: {}
fn wrap_macro_args (context : & RewriteContext < '_ > , args : & [ParsedMacroArg] , shape : Shape ,) -> RewriteResult { wrap_macro_args_inner (context , args , shape , false) . or_else (| _ | wrap_macro_args_inner (context , args , shape , true)) }
};
}
