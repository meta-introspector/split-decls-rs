// Generated macro for expand_combined_early_lint_pass_method (macro)
macro_rules! Depcrate_passesexpand_combined_early_lint_pass_method {
() => {
// Module: crate::passes
// Provides: {"expand_combined_early_lint_pass_method"}
// Dependencies: {}
# [macro_export] macro_rules ! expand_combined_early_lint_pass_method { ([$ ($ pass : ident) ,*] , $ self : ident , $ name : ident , $ params : tt) => ({ $ ($ self .$ pass .$ name $ params ;) * }) }
};
}
