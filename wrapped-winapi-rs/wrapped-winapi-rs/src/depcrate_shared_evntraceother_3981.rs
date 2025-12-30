// Generated macro for other_3981 (other)
macro_rules! Depcrate_shared_evntraceother_3981 {
() => {
// Module: crate::shared::evntrace
// Provides: {"other_3981"}
// Dependencies: {}
extern "C" { pub fn TraceMessage (SessionHandle : TRACEHANDLE , MessageFlags : ULONG , MessageGuid : LPGUID , MessageNumber : USHORT , ...) -> ULONG ; pub fn TraceMessageVa (SessionHandle : TRACEHANDLE , MessageFlags : ULONG , MessageGuid : LPGUID , MessageNumber : USHORT , MessageArgList : va_list ,) ; }
};
}
