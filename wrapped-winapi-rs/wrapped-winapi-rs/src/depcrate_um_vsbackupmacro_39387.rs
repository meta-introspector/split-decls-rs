// Generated macro for macro_39387 (macro)
macro_rules! Depcrate_um_vsbackupmacro_39387 {
() => {
// Module: crate::um::vsbackup
// Provides: {"macro_39387"}
// Dependencies: {}
RIDL ! { # [uuid (0x963f03ad , 0x9e4c , 0x4a34 , 0xac , 0x15 , 0xe4 , 0xb6 , 0x17 , 0x4e , 0x50 , 0x36)] interface IVssBackupComponentsEx (IVssBackupComponentsExVtbl) : IVssBackupComponents (IVssBackupComponentsVtbl) { fn GetWriterMetadataEx (iWriter : UINT , pidInstance : * mut VSS_ID , ppMetadata : * mut * mut IVssExamineWriterMetadataEx ,) -> HRESULT , fn SetSelectedForRestoreEx (writerId : VSS_ID , ct : VSS_COMPONENT_TYPE , wszLogicalPath : LPCWSTR , wszComponentName : LPCWSTR , bSelectedForRestore : bool , instanceId : VSS_ID ,) -> HRESULT , } }
};
}
