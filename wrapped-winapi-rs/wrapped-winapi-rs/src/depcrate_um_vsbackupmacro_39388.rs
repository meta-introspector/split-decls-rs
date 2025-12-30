// Generated macro for macro_39388 (macro)
macro_rules! Depcrate_um_vsbackupmacro_39388 {
() => {
// Module: crate::um::vsbackup
// Provides: {"macro_39388"}
// Dependencies: {}
RIDL ! { # [uuid (0xacfe2b3a , 0x22c9 , 0x4ef8 , 0xbd , 0x03 , 0x2f , 0x9c , 0xa2 , 0x30 , 0x08 , 0x4e)] interface IVssBackupComponentsEx2 (IVssBackupComponentsEx2Vtbl) : IVssBackupComponentsEx (IVssBackupComponentsExVtbl) { fn UnexposeSnapshot (snapshotId : VSS_ID ,) -> HRESULT , fn SetAuthoritativeRestore (writerId : VSS_ID , ct : VSS_COMPONENT_TYPE , wszLogicalPath : LPCWSTR , wszComponentName : LPCWSTR , bAuth : bool ,) -> HRESULT , fn SetRollForward (writerId : VSS_ID , ct : VSS_COMPONENT_TYPE , wszLogicalPath : LPCWSTR , wszComponentName : LPCWSTR , rollType : VSS_ROLLFORWARD_TYPE , wszRollForwardPoint : LPCWSTR ,) -> HRESULT , fn SetRestoreName (writerId : VSS_ID , ct : VSS_COMPONENT_TYPE , wszLogicalPath : LPCWSTR , wszComponentName : LPCWSTR , wszRestoreName : LPCWSTR ,) -> HRESULT , fn BreakSnapshotSetEx (SnapshotSetID : VSS_ID , dwBreakFlags : DWORD , ppAsync : * mut * mut IVssAsync ,) -> HRESULT , fn PreFastRecovery (SnapshotSetID : VSS_ID , dwPreFastRecoveryFlags : DWORD , ppAsync : * mut * mut IVssAsync ,) -> HRESULT , fn FastRecovery (SnapshotSetID : VSS_ID , dwFastRecoveryFlags : DWORD , ppAsync : * mut * mut IVssAsync ,) -> HRESULT , } }
};
}
