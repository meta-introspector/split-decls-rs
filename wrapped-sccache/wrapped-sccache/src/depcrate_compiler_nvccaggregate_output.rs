// Generated macro for aggregate_output (function)
macro_rules! Depcrate_compiler_nvccaggregate_output {
() => {
// Module: crate::compiler::nvcc
// Provides: {"aggregate_output"}
// Dependencies: {}
fn aggregate_output (lhs : process :: Output , rhs : process :: Output) -> process :: Output { process :: Output { status : exit_status (std :: cmp :: max (status_to_code (lhs . status) , status_to_code (rhs . status)) as ExitStatusValue ,) , stdout : [lhs . stdout , rhs . stdout] . concat () , stderr : [lhs . stderr , rhs . stderr] . concat () , } }
};
}
