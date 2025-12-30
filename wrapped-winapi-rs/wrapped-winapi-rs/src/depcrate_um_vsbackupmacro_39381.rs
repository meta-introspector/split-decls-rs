// Generated macro for macro_39381 (macro)
macro_rules! Depcrate_um_vsbackupmacro_39381 {
() => {
// Module: crate::um::vsbackup
// Provides: {"macro_39381"}
// Dependencies: {}
RIDL ! { # [uuid (0x902fcf7f , 0xb7fd , 0x42f8 , 0x81 , 0xf1 , 0xb2 , 0xe4 , 0x00 , 0xb1 , 0xe5 , 0xbd)] interface IVssExamineWriterMetadata (IVssExamineWriterMetadataVtbl) : IUnknown (IUnknownVtbl) { fn GetIdentity (pidInstance : * mut VSS_ID , pidWriter : * mut VSS_ID , pbstrWriterName : * mut BSTR , pUsage : * mut VSS_USAGE_TYPE , pSource : * mut VSS_SOURCE_TYPE ,) -> HRESULT , fn GetFileCounts (pcIncludeFiles : * mut UINT , pcExcludeFiles : * mut UINT , pcComponents : * mut UINT ,) -> HRESULT , fn GetIncludeFile (iFile : UINT , ppFiledesc : * mut * mut IVssWMFiledesc ,) -> HRESULT , fn GetExcludeFile (iFile : UINT , ppFiledesc : * mut * mut IVssWMFiledesc ,) -> HRESULT , fn GetComponent (iComponent : UINT , ppComponent : * mut * mut IVssWMComponent ,) -> HRESULT , fn GetRestoreMethod (pMethod : * mut VSS_RESTOREMETHOD_ENUM , pbstrService : * mut BSTR , pbstrUserProcedure : * mut BSTR , pwriterRestore : * mut VSS_WRITERRESTORE_ENUM , pbRebootRequired : * mut bool , pcMappings : * mut UINT ,) -> HRESULT , fn GetAlternateLocationMapping (iMapping : UINT , ppFiledesc : * mut * mut IVssWMFiledesc ,) -> HRESULT , fn GetBackupSchema (pdwSchemaMask : * mut DWORD ,) -> HRESULT , fn GetDocument (pDoc : * mut c_void ,) -> HRESULT , fn SaveAsXML (pbstrXML : * mut BSTR ,) -> HRESULT , fn LoadFromXML (pbstrXML : * mut BSTR ,) -> HRESULT , } }
};
}
