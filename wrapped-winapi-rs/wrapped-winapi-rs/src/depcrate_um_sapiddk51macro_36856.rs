// Generated macro for macro_36856 (macro)
macro_rules! Depcrate_um_sapiddk51macro_36856 {
() => {
// Module: crate::um::sapiddk51
// Provides: {"macro_36856"}
// Dependencies: {}
RIDL ! { # [uuid (0x2baeef81 , 0x2ca3 , 0x4331 , 0x98 , 0xf3 , 0x26 , 0xec , 0x5a , 0xbe , 0xfb , 0x03)] interface ISpTaskManager (ISpTaskManagerVtbl) : IUnknown (IUnknownVtbl) { fn SetThreadPoolInfo (pPoolInfo : * const SPTMTHREADINFO ,) -> HRESULT , fn GetThreadPoolInfo (pPoolInfo : * mut SPTMTHREADINFO ,) -> HRESULT , fn QueueTask (pTask : * mut ISpTask , pvTaskData : * mut c_void , hCompEvent : HANDLE , pdwGroupId : * mut DWORD , pTaskID : * mut DWORD ,) -> HRESULT , fn CreateReoccurringTask (pTask : * mut ISpTask , pvTaskData : * mut c_void , hCompEvent : HANDLE , ppTaskCtrl : * mut * mut ISpNotifySink ,) -> HRESULT , fn CreateThreadControl (pTask : * mut ISpThreadTask , pvTaskData : * mut c_void , nPriority : c_long , ppTaskCtrl : * mut * mut ISpThreadControl ,) -> HRESULT , fn TerminateTask (dwGroupId : DWORD , ulWaitPeriod : ULONG ,) -> HRESULT , } }
};
}
