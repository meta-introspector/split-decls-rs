// Generated macro for other_28600 (other)
macro_rules! Depcrate_um_evntconsother_28600 {
() => {
// Module: crate::um::evntcons
// Provides: {"other_28600"}
// Dependencies: {}
extern "system" { pub fn EventAccessControl (Guid : LPGUID , Operation : ULONG , Sid : PSID , Rights : ULONG , AllowOrDeny : BOOLEAN ,) -> ULONG ; pub fn EventAccessQuery (Guid : LPGUID , Buffer : PSECURITY_DESCRIPTOR , BufferSize : PULONG ,) -> ULONG ; pub fn EventAccessRemove (Guid : LPGUID ,) -> ULONG ; }
};
}
