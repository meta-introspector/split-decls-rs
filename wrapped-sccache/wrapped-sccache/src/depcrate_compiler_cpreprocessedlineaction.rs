// Generated macro for PreprocessedLineAction (type)
macro_rules! Depcrate_compiler_cPreprocessedLineAction {
() => {
// Module: crate::compiler::c
// Provides: {"PreprocessedLineAction"}
// Dependencies: {}
# [doc = " What to do after handling a preprocessor number line."] # [doc = " The `Break` variant is `(start, hash_start, continue_preprocessor_cache_mode)`."] # [doc = " The `Continue` variant is `(start, hash_start)`."] type PreprocessedLineAction = ControlFlow < (usize , usize , bool) , (usize , usize) > ;
};
}
