// Generated macro for other_34504 (other)
macro_rules! Depcrate_um_processsnapshotother_34504 {
() => {
// Module: crate::um::processsnapshot
// Provides: {"other_34504"}
// Dependencies: {}
extern "system" { pub fn PssCaptureSnapshot (ProcessHandle : HANDLE , CaptureFlags : PSS_CAPTURE_FLAGS , ThreadContextFlags : DWORD , SnapshotHandle : * mut HPSS ,) -> DWORD ; pub fn PssDuplicateSnapshot (SourceProcessHandle : HANDLE , SnapshotHandle : HPSS , TargetProcessHandle : HANDLE , TargetSnapshotHandle : * mut HPSS , Flags : PSS_DUPLICATE_FLAGS ,) -> DWORD ; pub fn PssFreeSnapshot (ProcessHandle : HANDLE , SnapshotHandle : HPSS ,) -> DWORD ; pub fn PssQuerySnapshot (SnapshotHandle : HPSS , InformationClass : PSS_QUERY_INFORMATION_CLASS , Buffer : * mut c_void , BufferLength : DWORD ,) -> DWORD ; pub fn PssWalkMarkerCreate (Allocator : * const PSS_ALLOCATOR , WalkMarkerHandle : * mut HPSSWALK ,) -> DWORD ; pub fn PssWalkMarkerFree (WalkMarkerHandle : HPSSWALK ,) -> DWORD ; pub fn PssWalkMarkerGetPosition (WalkMarkerHandle : HPSSWALK , Position : * mut ULONG_PTR ,) -> DWORD ; pub fn PssWalkMarkerSeekToBeginning (WalkMarkerHandle : HPSS ,) -> DWORD ; pub fn PssWalkMarkerSetPosition (WalkMarkerHandle : HPSSWALK , Position : ULONG_PTR ,) -> DWORD ; pub fn PssWalkSnapshot (SnapshotHandle : HPSS , InformationClass : PSS_WALK_INFORMATION_CLASS , WalkMarkerHandle : HPSSWALK , Buffer : * mut c_void , BufferLength : DWORD ,) -> DWORD ; }
};
}
