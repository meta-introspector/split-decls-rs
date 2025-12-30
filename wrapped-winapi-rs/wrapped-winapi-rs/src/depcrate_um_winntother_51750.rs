// Generated macro for other_51750 (other)
macro_rules! Depcrate_um_winntother_51750 {
() => {
// Module: crate::um::winnt
// Provides: {"other_51750"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] extern "system" { pub fn RtlAddFunctionTable (FunctionTable : PRUNTIME_FUNCTION , EntryCount : DWORD , BaseAddress : DWORD64 ,) -> BOOLEAN ; pub fn RtlDeleteFunctionTable (FunctionTable : PRUNTIME_FUNCTION ,) -> BOOLEAN ; pub fn RtlInstallFunctionTableCallback (TableIdentifier : DWORD64 , BaseAddress : DWORD64 , Length : DWORD , Callback : PGET_RUNTIME_FUNCTION_CALLBACK , Context : PVOID , OutOfProcessCallbackDll : PCWSTR ,) -> BOOLEAN ; pub fn RtlAddGrowableFunctionTable (DynamicTable : * mut PVOID , FunctionTable : PRUNTIME_FUNCTION , EntryCount : DWORD , MaximumEntryCount : DWORD , RangeBase : ULONG_PTR , RangeEnd : ULONG_PTR ,) -> DWORD ; pub fn RtlGrowFunctionTable (DynamicTable : PVOID , NewEntryCount : DWORD ,) ; pub fn RtlDeleteGrowableFunctionTable (DynamicTable : PVOID ,) ; pub fn RtlLookupFunctionEntry (ControlPc : DWORD64 , ImageBase : PDWORD64 , HistoryTable : PUNWIND_HISTORY_TABLE ,) -> PRUNTIME_FUNCTION ; }
};
}
