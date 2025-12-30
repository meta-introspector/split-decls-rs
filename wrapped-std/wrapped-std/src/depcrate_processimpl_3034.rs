// Generated macro for impl_3034 (impl)
macro_rules! Depcrate_processimpl_3034 {
() => {
// Module: crate::process
// Provides: {"impl_3034"}
// Dependencies: {}
# [stable (feature = "termination_trait_lib" , since = "1.61.0")] impl < T : Termination , E : fmt :: Debug > Termination for Result < T , E > { fn report (self) -> ExitCode { match self { Ok (val) => val . report () , Err (err) => { io :: attempt_print_to_stderr (format_args_nl ! ("Error: {err:?}")) ; ExitCode :: FAILURE } } } }
};
}
