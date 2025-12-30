// Generated macro for impl_1320 (impl)
macro_rules! Depcrate_netimpl_1320 {
() => {
// Module: crate::net
// Provides: {"impl_1320"}
// Dependencies: {}
impl SocketAddr { # [doc = " Get a Net address that with IP part set to \"127.0.0.1\"."] # [inline] pub fn with_port (port : u16) -> Self { SocketAddr :: Net (std :: net :: SocketAddr :: from (([127 , 0 , 0 , 1] , port))) } # [inline] pub fn as_net (& self) -> Option < & std :: net :: SocketAddr > { match self { SocketAddr :: Net (addr) => Some (addr) , # [cfg (unix)] _ => None , } } # [doc = " Parse a string as a unix domain socket."] # [doc = ""] # [doc = " The string should follow the format of `self.to_string()`."] # [cfg (unix)] pub fn parse_uds (s : & str) -> std :: io :: Result < Self > { # [cfg (any (target_os = "linux" , target_os = "android"))] { if s . starts_with ("\\x00") { let data = crate :: util :: ascii_unescape_default (& s . as_bytes () [4 ..]) ? ; return Ok (SocketAddr :: UnixAbstract (data)) ; } } let path = std :: path :: PathBuf :: from (s) ; Ok (SocketAddr :: Unix (path)) } # [cfg (unix)] pub fn is_unix_path (& self) -> bool { matches ! (self , SocketAddr :: Unix (_)) } # [cfg (not (unix))] pub fn is_unix_path (& self) -> bool { false } }
};
}
