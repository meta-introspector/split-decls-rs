// Generated macro for SocketAddr (enum)
macro_rules! Depcrate_netSocketAddr {
() => {
// Module: crate::net
// Provides: {"SocketAddr"}
// Dependencies: {}
# [derive (Debug)] pub enum SocketAddr { Net (std :: net :: SocketAddr) , # [cfg (unix)] Unix (std :: path :: PathBuf) , # [cfg (any (target_os = "linux" , target_os = "android"))] UnixAbstract (Vec < u8 >) , }
};
}
