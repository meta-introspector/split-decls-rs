// Generated macro for impl_122 (impl)
macro_rules! Depcrate_errorimpl_122 {
() => {
// Module: crate::error
// Provides: {"impl_122"}
// Dependencies: {}
impl From < toml :: ser :: Error > for Error { fn from (err : toml :: ser :: Error) -> Self { Error :: TomlSer (err) } }
};
}
