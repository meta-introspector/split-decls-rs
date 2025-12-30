// Generated macro for trace_header (macro)
macro_rules! Depcrate_trace_headertrace_header {
() => {
// Module: crate::trace_header
// Provides: {"trace_header"}
// Dependencies: {}
# [doc = " Macro to generate trace header with current location"] # [macro_export] macro_rules ! trace_header { ($ operation : expr , $ input : expr , $ processor : expr) => { $ crate :: trace_header :: generate_trace_header ($ operation , $ input , $ processor , & format ! ("{}:{}" , file ! () , line ! ())) } ; }
};
}
