// Generated macro for macro_19893 (macro)
macro_rules! Depcrate_um_bitsmacro_19893 {
() => {
// Module: crate::um::bits
// Provides: {"macro_19893"}
// Dependencies: {}
RIDL ! { # [uuid (0xca29d251 , 0xb4bb , 0x4679 , 0xa3 , 0xd9 , 0xae , 0x80 , 0x06 , 0x11 , 0x9d , 0x54)] interface AsyncIBackgroundCopyCallback (AsyncIBackgroundCopyCallbackVtbl) : IUnknown (IUnknownVtbl) { fn Begin_JobTransferred (pJob : * mut IBackgroundCopyJob ,) -> HRESULT , fn Finish_JobTransferred () -> HRESULT , fn Begin_JobError (pJob : * mut IBackgroundCopyJob , pError : * mut IBackgroundCopyError ,) -> HRESULT , fn Finish_JobError () -> HRESULT , fn Begin_JobModification (pJob : * mut IBackgroundCopyJob , dwReserved : DWORD ,) -> HRESULT , fn Finish_JobModification () -> HRESULT , } }
};
}
