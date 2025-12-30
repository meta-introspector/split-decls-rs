// Generated macro for other_3964 (other)
macro_rules! Depcrate_shared_evntraceother_3964 {
() => {
// Module: crate::shared::evntrace
// Provides: {"other_3964"}
// Dependencies: {}
extern "system" { pub fn EnableTraceEx2 (TraceHandle : TRACEHANDLE , ProviderId : LPCGUID , ControlCode : ULONG , Level : UCHAR , MatchAnyKeyword : ULONGLONG , MatchAllKeyword : ULONGLONG , Timeout : ULONG , EnableParameters : PENABLE_TRACE_PARAMETERS ,) -> ULONG ; }
};
}
