// Generated macro for macro_39389 (macro)
macro_rules! Depcrate_um_vsbackupmacro_39389 {
() => {
// Module: crate::um::vsbackup
// Provides: {"macro_39389"}
// Dependencies: {}
RIDL ! { # [uuid (0xc191bfbc , 0xb602 , 0x4675 , 0x8b , 0xd1 , 0x67 , 0xd6 , 0x42 , 0xf5 , 0x29 , 0xd5)] interface IVssBackupComponentsEx3 (IVssBackupComponentsEx3Vtbl) : IVssBackupComponentsEx2 (IVssBackupComponentsEx2Vtbl) { fn GetWriterStatusEx (iWriter : UINT , pidInstance : * mut VSS_ID , pidWriter : * mut VSS_ID , pbstrWriter : * mut BSTR , pnStatus : * mut VSS_WRITER_STATE , phrFailureWriter : * mut HRESULT , phrApplication : * mut HRESULT , pbstrApplicationMessage : * mut BSTR ,) -> HRESULT , fn AddSnapshotToRecoverySet (snapshotId : VSS_ID , dwFlags : DWORD , pwszDestinationVolume : VSS_PWSZ ,) -> HRESULT , fn RecoverSet (dwFlags : DWORD , ppAsync : * mut * mut IVssAsync ,) -> HRESULT , fn GetSessionId (idSession : * mut VSS_ID ,) -> HRESULT , } }
};
}
