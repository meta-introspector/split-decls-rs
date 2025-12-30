// Generated macro for other_39392 (other)
macro_rules! Depcrate_um_vsbackupother_39392 {
() => {
// Module: crate::um::vsbackup
// Provides: {"other_39392"}
// Dependencies: {}
extern "system" { # [link_name = "CreateVssBackupComponentsInternal"] pub fn CreateVssBackupComponents (ppBackup : * mut * mut IVssBackupComponents ,) -> HRESULT ; # [link_name = "CreateVssExamineWriterMetadataInternal"] pub fn CreateVssExamineWriterMetadata (bstrXML : BSTR , ppMetadata : * mut * mut IVssExamineWriterMetadata ,) -> HRESULT ; # [link_name = "IsVolumeSnapshottedInternal"] pub fn IsVolumeSnapshotted (pwszVolumeName : VSS_PWSZ , pbSnapshotsPresent : * mut BOOL , plSnapshotCapability : * mut LONG ,) -> HRESULT ; # [link_name = "VssFreeSnapshotPropertiesInternal"] pub fn VssFreeSnapshotProperties (pProp : * mut VSS_SNAPSHOT_PROP ,) ; # [link_name = "GetProviderMgmtInterfaceInternal"] pub fn GetProviderMgmtInterface (ProviderId : VSS_ID , InterfaceId : IID , ppItf : * mut * mut IUnknown ,) -> HRESULT ; # [link_name = "ShouldBlockRevertInternal"] pub fn ShouldBlockRevert (wszVolumeName : LPCWSTR , pbBlock : * mut bool ,) -> HRESULT ; }
};
}
