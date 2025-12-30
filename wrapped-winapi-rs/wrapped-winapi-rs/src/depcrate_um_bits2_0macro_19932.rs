// Generated macro for macro_19932 (macro)
macro_rules! Depcrate_um_bits2_0macro_19932 {
() => {
// Module: crate::um::bits2_0
// Provides: {"macro_19932"}
// Dependencies: {}
RIDL ! { # [uuid (0x443c8934 , 0x90ff , 0x48ed , 0xbc , 0xde , 0x26 , 0xf5 , 0xc7 , 0x45 , 0x00 , 0x42)] interface IBackgroundCopyJob3 (IBackgroundCopyJob3Vtbl) : IBackgroundCopyJob2 (IBackgroundCopyJob2Vtbl) { fn ReplaceRemotePrefix (OldPrefix : LPCWSTR , NewPrefix : LPCWSTR ,) -> HRESULT , fn AddFileWithRanges (RemoteUrl : LPCWSTR , LocalName : LPCWSTR , RangeCount : DWORD , Ranges : * mut BG_FILE_RANGE ,) -> HRESULT , fn SetFileACLFlags (Flags : DWORD ,) -> HRESULT , fn GetFileACLFlags (Flags : * mut DWORD ,) -> HRESULT , } }
};
}
