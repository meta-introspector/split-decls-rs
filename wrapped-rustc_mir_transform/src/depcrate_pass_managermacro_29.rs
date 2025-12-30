// Generated macro for macro_29 (macro)
macro_rules! Depcrate_pass_managermacro_29 {
() => {
// Module: crate::pass_manager
// Provides: {"macro_29"}
// Dependencies: {}
thread_local ! { # [doc = " Maps MIR pass names to a snake case form to match profiling naming style"] static PASS_TO_PROFILER_NAMES : RefCell < FxHashMap <&'static str , &'static str >> = { RefCell :: new (FxHashMap :: default ()) } ; }
};
}
