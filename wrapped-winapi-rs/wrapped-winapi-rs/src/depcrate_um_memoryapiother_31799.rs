// Generated macro for other_31799 (other)
macro_rules! Depcrate_um_memoryapiother_31799 {
() => {
// Module: crate::um::memoryapi
// Provides: {"other_31799"}
// Dependencies: {}
extern "system" { pub fn PrefetchVirtualMemory (hProcess : HANDLE , NumberOfEntries : ULONG_PTR , VirtualAddresses : PWIN32_MEMORY_RANGE_ENTRY , Flags : ULONG ,) -> BOOL ; pub fn CreateFileMappingFromApp (hFile : HANDLE , SecurityAttributes : PSECURITY_ATTRIBUTES , PageProtection : ULONG , MaximumSize : ULONG64 , Name : PCWSTR ,) -> HANDLE ; pub fn MapViewOfFileFromApp (hFileMappingObject : HANDLE , DesiredAccess : ULONG , FileOffset : ULONG64 , NumberOfBytesToMap : SIZE_T ,) -> PVOID ; pub fn UnmapViewOfFileEx (BaseAddress : PVOID , UnmapFlags : ULONG ,) -> BOOL ; pub fn AllocateUserPhysicalPages (hProcess : HANDLE , NumberOfPages : PULONG_PTR , PageArray : PULONG_PTR ,) -> BOOL ; pub fn FreeUserPhysicalPages (hProcess : HANDLE , NumberOfPages : PULONG_PTR , PageArray : PULONG_PTR ,) -> BOOL ; pub fn MapUserPhysicalPages (VirtualAddress : PVOID , NumberOfPages : ULONG_PTR , PageArray : PULONG_PTR ,) -> BOOL ; pub fn AllocateUserPhysicalPagesNuma (hProcess : HANDLE , NumberOfPages : PULONG_PTR , PageArray : PULONG_PTR , nndPreferred : DWORD ,) -> BOOL ; pub fn VirtualAllocExNuma (hProcess : HANDLE , lpAddress : LPVOID , dwSize : SIZE_T , flAllocationType : DWORD , flProtect : DWORD , nndPreferred : DWORD ,) -> LPVOID ; }
};
}
