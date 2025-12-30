// Generated macro for macro_51751 (macro)
macro_rules! Depcrate_um_winntmacro_51751 {
() => {
// Module: crate::um::winnt
// Provides: {"macro_51751"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] IFDEF ! { extern "C" { pub fn RtlRestoreContext (ContextRecord : PCONTEXT , ExceptionRecord : * mut EXCEPTION_RECORD ,) ; } extern "system" { pub fn RtlUnwindEx (TargetFrame : PVOID , TargetIp : PVOID , ExceptionRecord : PEXCEPTION_RECORD , ReturnValue : PVOID , ContextRecord : PCONTEXT , HistoryTable : PUNWIND_HISTORY_TABLE ,) ; pub fn RtlVirtualUnwind (HandlerType : DWORD , ImageBase : DWORD64 , ControlPc : DWORD64 , FunctionEntry : PRUNTIME_FUNCTION , ContextRecord : PCONTEXT , HandlerData : * mut PVOID , EstablisherFrame : PDWORD64 , ContextPointers : PKNONVOLATILE_CONTEXT_POINTERS ,) -> PEXCEPTION_ROUTINE ; } }
};
}
