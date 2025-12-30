// Generated macro for other_51749 (other)
macro_rules! Depcrate_um_winntother_51749 {
() => {
// Module: crate::um::winnt
// Provides: {"other_51749"}
// Dependencies: {}
extern "system" { pub fn RtlCaptureStackBackTrace (FramesToSkip : DWORD , FramesToCapture : DWORD , BackTrace : * mut PVOID , BackTraceHash : PDWORD ,) -> WORD ; pub fn RtlCaptureContext (ContextRecord : PCONTEXT ,) ; pub fn RtlUnwind (TargetFrame : PVOID , TargetIp : PVOID , ExceptionRecord : PEXCEPTION_RECORD , ReturnValue : PVOID ,) ; }
};
}
