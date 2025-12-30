// Generated macro for other_32378 (other)
macro_rules! Depcrate_um_mswsockother_32378 {
() => {
// Module: crate::um::mswsock
// Provides: {"other_32378"}
// Dependencies: {}
extern "system" { pub fn TransmitFile (hSocket : SOCKET , hFile : HANDLE , nNumberOfBytesToWrite : DWORD , nNumberOfBytesPerSend : DWORD , lpOverlapped : LPOVERLAPPED , lpTransmitBuffers : LPTRANSMIT_FILE_BUFFERS , dwReserved : DWORD ,) -> BOOL ; pub fn AcceptEx (sListenSocket : SOCKET , sAcceptSocket : SOCKET , lpOutputBuffer : PVOID , dwReceiveDataLength : DWORD , dwLocalAddressLength : DWORD , dwRemoteAddressLength : DWORD , lpdwBytesReceived : LPDWORD , lpOverlapped : LPOVERLAPPED ,) -> BOOL ; pub fn GetAcceptExSockaddrs (lpOutputBuffer : PVOID , dwReceiveDataLength : DWORD , dwLocalAddressLength : DWORD , dwRemoteAddressLength : DWORD , LocalSockaddr : * mut * mut SOCKADDR , LocalSockaddrLength : LPINT , RemoteSockaddr : * mut * mut SOCKADDR , RemoteSockaddrLength : LPINT ,) ; }
};
}
