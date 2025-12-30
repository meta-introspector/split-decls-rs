// Generated macro for other_29074 (other)
macro_rules! Depcrate_um_ioapisetother_29074 {
() => {
// Module: crate::um::ioapiset
// Provides: {"other_29074"}
// Dependencies: {}
extern "system" { pub fn CreateIoCompletionPort (FileHandle : HANDLE , ExistingCompletionPort : HANDLE , CompletionKey : ULONG_PTR , NumberOfConcurrentThreads : DWORD ,) -> HANDLE ; pub fn GetQueuedCompletionStatus (CompletionPort : HANDLE , lpNumberOfBytesTransferred : LPDWORD , lpCompletionKey : PULONG_PTR , lpOverlapped : * mut LPOVERLAPPED , dwMilliseconds : DWORD ,) -> BOOL ; pub fn GetQueuedCompletionStatusEx (CompletionPort : HANDLE , lpCompletionPortEntries : LPOVERLAPPED_ENTRY , ulCount : ULONG , ulNumEntriesRemoved : PULONG , dwMilliseconds : DWORD , fAlertable : BOOL ,) -> BOOL ; pub fn PostQueuedCompletionStatus (CompletionPort : HANDLE , dwNumberOfBytesTransferred : DWORD , dwCompletionKey : ULONG_PTR , lpOverlapped : LPOVERLAPPED ,) -> BOOL ; pub fn DeviceIoControl (hDevice : HANDLE , dwIoControlCode : DWORD , lpInBuffer : LPVOID , nInBufferSize : DWORD , lpOutBuffer : LPVOID , nOutBufferSize : DWORD , lpBytesReturned : LPDWORD , lpOverlapped : LPOVERLAPPED ,) -> BOOL ; pub fn GetOverlappedResult (hFile : HANDLE , lpOverlapped : LPOVERLAPPED , lpNumberOfBytesTransferred : LPDWORD , bWait : BOOL ,) -> BOOL ; pub fn CancelIoEx (hFile : HANDLE , lpOverlapped : LPOVERLAPPED ,) -> BOOL ; pub fn CancelIo (hFile : HANDLE ,) -> BOOL ; pub fn GetOverlappedResultEx (hFile : HANDLE , lpOverlapped : LPOVERLAPPED , lpNumberOfBytesTransferred : LPDWORD , dwMilliseconds : DWORD , bAlertable : BOOL ,) -> BOOL ; pub fn CancelSynchronousIo (hThread : HANDLE ,) -> BOOL ; }
};
}
