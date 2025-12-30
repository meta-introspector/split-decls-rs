// Generated macro for other_3980 (other)
macro_rules! Depcrate_shared_evntraceother_3980 {
() => {
// Module: crate::shared::evntrace
// Provides: {"other_3980"}
// Dependencies: {}
extern "system" { pub fn QueryTraceProcessingHandle (ProcessingHandle : TRACEHANDLE , InformationClass : ETW_PROCESS_HANDLE_INFO_TYPE , InBuffer : PVOID , InBufferSize : ULONG , OutBuffer : PVOID , OutBufferSize : ULONG , ReturnLength : PULONG ,) -> ULONG ; pub fn OpenTraceA (Logfile : PEVENT_TRACE_LOGFILEA ,) -> TRACEHANDLE ; pub fn SetTraceCallback (pGuid : LPCGUID , EventCallback : PEVENT_CALLBACK ,) -> ULONG ; pub fn RemoveTraceCallback (pGuid : LPCGUID ,) -> ULONG ; }
};
}
