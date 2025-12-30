// Generated macro for AppDataRecord (struct)
macro_rules! Depcrate_conn_unbufferedAppDataRecord {
() => {
// Module: crate::conn::unbuffered
// Provides: {"AppDataRecord"}
// Dependencies: {}
# [doc = " A decrypted application-data record"] # [non_exhaustive] pub struct AppDataRecord < 'i > { # [doc = " Number of additional bytes to discard"] # [doc = ""] # [doc = " This number MUST be added to the value of [`UnbufferedStatus::discard`] *prior* to the"] # [doc = " discard operation. See [`UnbufferedStatus::discard`] for more details"] pub discard : usize , # [doc = " The payload of the app-data record"] pub payload : & 'i [u8] , }
};
}
