// Generated macro for LSAP_SE_ADT_PARAMETER_ARRAY_TRUE_SIZE (function)
macro_rules! Depcrate_um_ntlsaLSAP_SE_ADT_PARAMETER_ARRAY_TRUE_SIZE {
() => {
// Module: crate::um::ntlsa
// Provides: {"LSAP_SE_ADT_PARAMETER_ARRAY_TRUE_SIZE"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] # [inline] pub fn LSAP_SE_ADT_PARAMETER_ARRAY_TRUE_SIZE (AuditParameters : SE_ADT_PARAMETER_ARRAY ,) -> SIZE_T { 1048 - (32 * (SE_MAX_AUDIT_PARAMETERS - AuditParameters . ParameterCount as SIZE_T)) }
};
}
