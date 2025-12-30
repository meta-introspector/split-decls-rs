// Generated macro for other_37765 (other)
macro_rules! Depcrate_um_setupapiother_37765 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37765"}
// Dependencies: {}
extern "system" { pub fn SetupCopyOEMInfA (SourceInfFileName : PCSTR , OEMSourceMediaLocation : PCSTR , OEMSourceMediaType : DWORD , CopyStyle : DWORD , DestinationInfFileName : PSTR , DestinationInfFileNameSize : DWORD , RequiredSize : PDWORD , DestinationInfFileNameComponent : * mut PSTR ,) -> BOOL ; pub fn SetupCopyOEMInfW (SourceInfFileName : PCWSTR , OEMSourceMediaLocation : PCWSTR , OEMSourceMediaType : DWORD , CopyStyle : DWORD , DestinationInfFileName : PWSTR , DestinationInfFileNameSize : DWORD , RequiredSize : PDWORD , DestinationInfFileNameComponent : * mut PWSTR ,) -> BOOL ; }
};
}
