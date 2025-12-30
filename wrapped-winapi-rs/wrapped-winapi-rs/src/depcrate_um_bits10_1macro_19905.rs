// Generated macro for macro_19905 (macro)
macro_rules! Depcrate_um_bits10_1macro_19905 {
() => {
// Module: crate::um::bits10_1
// Provides: {"macro_19905"}
// Dependencies: {}
RIDL ! { # [uuid (0xcf6784f7 , 0xd677 , 0x49fd , 0x93 , 0x68 , 0xcb , 0x47 , 0xae , 0xe9 , 0xd1 , 0xad)] interface IBackgroundCopyFile6 (IBackgroundCopyFile6Vtbl) : IBackgroundCopyFile5 (IBackgroundCopyFile5Vtbl) { fn UpdateDownloadPosition (offset : UINT64 ,) -> HRESULT , fn RequestFileRanges (rangeCount : DWORD , ranges : * const BG_FILE_RANGE ,) -> HRESULT , fn GetFilledFileRanges (rangeCount : * mut DWORD , ranges : * mut * mut BG_FILE_RANGE ,) -> HRESULT , } }
};
}
