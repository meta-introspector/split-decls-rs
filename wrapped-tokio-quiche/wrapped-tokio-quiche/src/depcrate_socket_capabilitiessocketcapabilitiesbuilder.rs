// Generated macro for SocketCapabilitiesBuilder (struct)
macro_rules! Depcrate_socket_capabilitiesSocketCapabilitiesBuilder {
() => {
// Module: crate::socket::capabilities
// Provides: {"SocketCapabilitiesBuilder"}
// Dependencies: {}
# [doc = " Builder to enable Linux sockopts which improve QUIC performance."] # [cfg (target_os = "linux")] pub struct SocketCapabilitiesBuilder < 's > { socket : BorrowedFd < 's > , cap : SocketCapabilities , }
};
}
