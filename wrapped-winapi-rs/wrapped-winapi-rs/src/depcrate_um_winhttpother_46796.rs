// Generated macro for other_46796 (other)
macro_rules! Depcrate_um_winhttpother_46796 {
() => {
// Module: crate::um::winhttp
// Provides: {"other_46796"}
// Dependencies: {}
extern "system" { pub fn WinHttpWebSocketCompleteUpgrade (hRequest : HINTERNET , pContext : DWORD_PTR ,) -> HINTERNET ; pub fn WinHttpWebSocketSend (hWebSocket : HINTERNET , eBufferType : WINHTTP_WEB_SOCKET_BUFFER_TYPE , pvBuffer : PVOID , dwBufferLength : DWORD ,) -> DWORD ; pub fn WinHttpWebSocketReceive (hWebSocket : HINTERNET , pvBuffer : PVOID , dwBufferLength : DWORD , pdwBytesRead : * mut DWORD , peBufferType : * mut WINHTTP_WEB_SOCKET_BUFFER_TYPE ,) -> DWORD ; pub fn WinHttpWebSocketShutdown (hWebSocket : HINTERNET , usStatus : USHORT , pvReason : PVOID , dwReasonLength : DWORD ,) -> DWORD ; pub fn WinHttpWebSocketClose (hWebSocket : HINTERNET , usStatus : USHORT , pvReason : PVOID , dwReasonLength : DWORD ,) -> DWORD ; pub fn WinHttpWebSocketQueryCloseStatus (hWebSocket : HINTERNET , pusStatus : * mut USHORT , pvReason : PVOID , dwReasonLength : DWORD , pdwReasonLengthConsumed : * mut DWORD ,) -> DWORD ; }
};
}
