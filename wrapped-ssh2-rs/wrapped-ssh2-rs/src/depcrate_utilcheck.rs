// Generated macro for check (function)
macro_rules! Depcrate_utilcheck {
() => {
// Module: crate::util
// Provides: {"check"}
// Dependencies: {}
fn check (b : Cow < [u8] >) -> Result < Cow < [u8] > , Error > { if b . iter () . any (| b | * b == 0) { Err (Error :: new (ErrorCode :: Session (raw :: LIBSSH2_ERROR_INVAL) , "path provided contains a 0 byte" ,)) } else { Ok (b) } }
};
}
