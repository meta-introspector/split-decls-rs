// Generated macro for CompilerProxyMap (type)
macro_rules! Depcrate_serverCompilerProxyMap {
() => {
// Module: crate::server
// Provides: {"CompilerProxyMap"}
// Dependencies: {}
# [doc = " Maps a compiler proxy path to a compiler proxy and it's last modification time"] type CompilerProxyMap < C > = HashMap < PathBuf , (Box < dyn CompilerProxy < C > > , FileTime) > ;
};
}
