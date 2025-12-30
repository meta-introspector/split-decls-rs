// Generated macro for macro_19970 (macro)
macro_rules! Depcrate_um_bits3_0macro_19970 {
() => {
// Module: crate::um::bits3_0
// Provides: {"macro_19970"}
// Dependencies: {}
RIDL ! { # [uuid (0x659cdeae , 0x489e , 0x11d9 , 0xa9 , 0xcd , 0x00 , 0x0d , 0x56 , 0x96 , 0x52 , 0x51)] interface IBackgroundCopyJob4 (IBackgroundCopyJob4Vtbl) : IBackgroundCopyJob3 (IBackgroundCopyJob3Vtbl) { fn SetPeerCachingFlags (Flags : DWORD ,) -> HRESULT , fn GetPeerCachingFlags (pFlags : * mut DWORD ,) -> HRESULT , fn GetOwnerIntegrityLevel (pLevel : * mut ULONG ,) -> HRESULT , fn GetOwnerElevationState (pElevated : * mut BOOL ,) -> HRESULT , fn SetMaximumDownloadTime (Timeout : ULONG ,) -> HRESULT , fn GetMaximumDownloadTime (pTimeout : * mut ULONG ,) -> HRESULT , } }
};
}
