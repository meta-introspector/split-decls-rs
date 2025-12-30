// Generated macro for other_39064 (other)
macro_rules! Depcrate_um_tlhelp32other_39064 {
() => {
// Module: crate::um::tlhelp32
// Provides: {"other_39064"}
// Dependencies: {}
extern "system" { pub fn Heap32First (lphe : LPHEAPENTRY32 , th32ProcessID : DWORD , th32HeapID : ULONG_PTR ,) -> BOOL ; pub fn Heap32Next (lphe : LPHEAPENTRY32 ,) -> BOOL ; pub fn Toolhelp32ReadProcessMemory (th32ProcessID : DWORD , lpBaseAddress : LPCVOID , lpBuffer : LPVOID , cbRead : SIZE_T , lpNumberOfBytesRead : * mut SIZE_T ,) -> BOOL ; }
};
}
