// Generated macro for other_30075 (other)
macro_rules! Depcrate_um_lmapibufother_30075 {
() => {
// Module: crate::um::lmapibuf
// Provides: {"other_30075"}
// Dependencies: {}
extern "system" { pub fn NetApiBufferAllocate (ByteCount : DWORD , Buffer : * mut LPVOID ,) -> NET_API_STATUS ; pub fn NetApiBufferFree (Buffer : LPVOID ,) -> NET_API_STATUS ; pub fn NetApiBufferReallocate (OldBuffer : LPVOID , NewByteCount : DWORD , NewBuffer : * mut LPVOID ,) -> NET_API_STATUS ; pub fn NetApiBufferSize (Buffer : LPVOID , ByteCount : LPDWORD ,) -> NET_API_STATUS ; pub fn NetapipBufferAllocate (ByteCount : DWORD , Buffer : * mut LPVOID ,) -> NET_API_STATUS ; }
};
}
