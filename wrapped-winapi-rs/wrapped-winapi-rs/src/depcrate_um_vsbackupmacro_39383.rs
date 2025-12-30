// Generated macro for macro_39383 (macro)
macro_rules! Depcrate_um_vsbackupmacro_39383 {
() => {
// Module: crate::um::vsbackup
// Provides: {"macro_39383"}
// Dependencies: {}
RIDL ! { # [uuid (0xce115780 , 0xa611 , 0x431b , 0xb5 , 0x7f , 0xc3 , 0x83 , 0x03 , 0xab , 0x6a , 0xee)] interface IVssExamineWriterMetadataEx2 (IVssExamineWriterMetadataEx2Vtbl) : IVssExamineWriterMetadataEx (IVssExamineWriterMetadataExVtbl) { fn GetVersion (pdwMajorVersion : * mut DWORD , pdwMinorVersion : * mut DWORD ,) -> HRESULT , fn GetExcludeFromSnapshotCount (pcExcludedFromSnapshot : * mut UINT ,) -> HRESULT , fn GetExcludeFromSnapshotFile (iFile : UINT , ppFiledesc : * mut * mut IVssWMFiledesc ,) -> HRESULT , } }
};
}
