// Generated macro for trace_result (function)
macro_rules! Depcrate_combinator_debugtrace_result {
() => {
// Module: crate::combinator::debug
// Provides: {"trace_result"}
// Dependencies: {}
# [cfg_attr (not (feature = "debug") , allow (unused_variables))] pub (crate) fn trace_result < T , I : Stream , E : ParserError < I > > (name : impl core :: fmt :: Display , res : & Result < T , E > ,) { # [cfg (feature = "debug")] { let depth = internals :: Depth :: existing () ; let severity = internals :: Severity :: with_result (res) ; internals :: result (* depth , & name , severity) ; } }
};
}
