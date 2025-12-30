// Generated macro for other_58249 (other)
macro_rules! Depcrate_um_ws2tcpipother_58249 {
() => {
// Module: crate::um::ws2tcpip
// Provides: {"other_58249"}
// Dependencies: {}
extern "system" { pub fn WSASetSocketSecurity (Socket : SOCKET , SecuritySettings : * const SOCKET_SECURITY_SETTINGS , SecuritySettingsLen : ULONG , Overlapped : LPWSAOVERLAPPED , CompletionRoutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE ,) -> INT ; pub fn WSAQuerySocketSecurity (Socket : SOCKET , SecurityQueryTemplate : * const SOCKET_SECURITY_QUERY_TEMPLATE , SecurityQueryTemplateLen : ULONG , SecurityQueryInfo : * mut SOCKET_SECURITY_QUERY_INFO , SecurityQueryInfoLen : * mut ULONG , Overlapped : LPWSAOVERLAPPED , CompletionRoutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE ,) -> INT ; pub fn WSASetSocketPeerTargetName (Socket : SOCKET , PeerTargetName : * const SOCKET_PEER_TARGET_NAME , PeerTargetNameLen : ULONG , Overlapped : LPWSAOVERLAPPED , CompletionRoutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE ,) -> INT ; pub fn WSADeleteSocketPeerTargetName (Socket : SOCKET , PeerAddr : * const SOCKADDR , PeerAddrLen : ULONG , Overlapped : LPWSAOVERLAPPED , CompletionRoutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE ,) -> INT ; pub fn WSAImpersonateSocketPeer (Socket : SOCKET , PeerAddr : * const SOCKADDR , PeerAddrLen : ULONG ,) -> INT ; pub fn WSARevertImpersonation () ; }
};
}
