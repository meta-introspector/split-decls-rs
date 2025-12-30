// Generated macro for other_37090 (other)
macro_rules! Depcrate_um_securityappcontainerother_37090 {
() => {
// Module: crate::um::securityappcontainer
// Provides: {"other_37090"}
// Dependencies: {}
extern "system" { pub fn GetAppContainerNamedObjectPath (Token : HANDLE , AppContainerSid : PSID , ObjectPathLength : ULONG , ObjectPath : LPWSTR , ReturnLength : PULONG ,) -> BOOL ; }
};
}
