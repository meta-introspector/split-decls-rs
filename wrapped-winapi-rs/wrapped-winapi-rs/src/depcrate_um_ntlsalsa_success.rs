// Generated macro for LSA_SUCCESS (function)
macro_rules! Depcrate_um_ntlsaLSA_SUCCESS {
() => {
// Module: crate::um::ntlsa
// Provides: {"LSA_SUCCESS"}
// Dependencies: {}
# [inline] pub fn LSA_SUCCESS (Error : NTSTATUS) -> bool { (Error as LONG) >= 0 }
};
}
