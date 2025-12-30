// Generated macro for cvt (function)
macro_rules! Depcratecvt {
() => {
// Module: crate
// Provides: {"cvt"}
// Dependencies: {}
# [inline (always)] fn cvt (err : OSStatus) -> Result < () > { match err { errSecSuccess => Ok (()) , err => Err (Error :: from_code (err)) , } }
};
}
