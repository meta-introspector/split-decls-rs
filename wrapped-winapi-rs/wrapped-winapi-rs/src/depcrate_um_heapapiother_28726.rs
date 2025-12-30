// Generated macro for other_28726 (other)
macro_rules! Depcrate_um_heapapiother_28726 {
() => {
// Module: crate::um::heapapi
// Provides: {"other_28726"}
// Dependencies: {}
extern "system" { pub fn HeapCreate (flOptions : DWORD , dwInitialSize : SIZE_T , dwMaximumSize : SIZE_T ,) -> HANDLE ; pub fn HeapDestroy (hHeap : HANDLE ,) -> BOOL ; pub fn HeapAlloc (hHeap : HANDLE , dwFlags : DWORD , dwBytes : SIZE_T ,) -> LPVOID ; pub fn HeapReAlloc (hHeap : HANDLE , dwFlags : DWORD , lpMem : LPVOID , dwBytes : SIZE_T ,) -> LPVOID ; pub fn HeapFree (hHeap : HANDLE , dwFlags : DWORD , lpMem : LPVOID ,) -> BOOL ; pub fn HeapSize (hHeap : HANDLE , dwFlags : DWORD , lpMem : LPCVOID ,) -> SIZE_T ; pub fn GetProcessHeap () -> HANDLE ; pub fn HeapCompact (hHeap : HANDLE , dwFlags : DWORD ,) -> SIZE_T ; pub fn HeapSetInformation (HeapHandle : HANDLE , HeapInformationClass : HEAP_INFORMATION_CLASS , HeapInformation : PVOID , HeapInformationLength : SIZE_T ,) -> BOOL ; pub fn HeapValidate (hHeap : HANDLE , dwFlags : DWORD , lpMem : LPCVOID ,) -> BOOL ; pub fn HeapSummary (hHeap : HANDLE , dwFlags : DWORD , lpSummary : LPHEAP_SUMMARY ,) -> BOOL ; pub fn GetProcessHeaps (NumberOfHeaps : DWORD , ProcessHeaps : PHANDLE ,) -> DWORD ; pub fn HeapLock (hHeap : HANDLE ,) -> BOOL ; pub fn HeapUnlock (hHeap : HANDLE ,) -> BOOL ; pub fn HeapWalk (hHeap : HANDLE , lpEntry : LPPROCESS_HEAP_ENTRY ,) -> BOOL ; pub fn HeapQueryInformation (HeapHandle : HANDLE , HeapInformationClass : HEAP_INFORMATION_CLASS , HeapInformation : PVOID , HeapInformationLength : SIZE_T , ReturnLength : PSIZE_T ,) -> BOOL ; }
};
}
