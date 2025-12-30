// Generated macro for RUSTVERSION (const)
macro_rules! DepcrateRUSTVERSION {
() => {
// Module: crate
// Provides: {"RUSTVERSION"}
// Dependencies: {}
# [cfg (host_os = "windows")] const RUSTVERSION : Version = include ! (concat ! (env ! ("OUT_DIR") , "\\version.expr")) ;
};
}
