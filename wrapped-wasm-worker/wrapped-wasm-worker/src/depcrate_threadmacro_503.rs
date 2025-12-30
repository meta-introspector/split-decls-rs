// Generated macro for macro_503 (macro)
macro_rules! Depcrate_threadmacro_503 {
() => {
// Module: crate::thread
// Provides: {"macro_503"}
// Dependencies: {}
thread_local ! { # [doc = " Holds this threads [`Thread`]."] static THREAD : OnceCell < Thread > = const { OnceCell :: new () } ; }
};
}
