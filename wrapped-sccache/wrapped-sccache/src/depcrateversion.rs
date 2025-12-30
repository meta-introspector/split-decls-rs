// Generated macro for VERSION (const)
macro_rules! DepcrateVERSION {
() => {
// Module: crate
// Provides: {"VERSION"}
// Dependencies: {}
# [doc = " VERSION is the pkg version of sccache."] # [doc = ""] # [doc = " This version is safe to be used in cache services to indicate the version"] # [doc = " that sccache ie."] pub const VERSION : & str = env ! ("CARGO_PKG_VERSION") ;
};
}
