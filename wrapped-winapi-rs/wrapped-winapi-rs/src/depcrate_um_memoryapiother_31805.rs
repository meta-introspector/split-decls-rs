// Generated macro for other_31805 (other)
macro_rules! Depcrate_um_memoryapiother_31805 {
() => {
// Module: crate::um::memoryapi
// Provides: {"other_31805"}
// Dependencies: {}
extern "system" { pub fn OfferVirtualMemory (VirtualAddress : PVOID , Size : SIZE_T , Priority : OFFER_PRIORITY ,) -> DWORD ; pub fn ReclaimVirtualMemory (VirtualAddress : * const c_void , Size : SIZE_T ,) -> DWORD ; pub fn DiscardVirtualMemory (VirtualAddress : PVOID , Size : SIZE_T ,) -> DWORD ; pub fn VirtualAllocFromApp (BaseAddress : PVOID , Size : SIZE_T , AllocationType : ULONG , Protection : ULONG ,) -> PVOID ; pub fn VirtualProtectFromApp (Address : PVOID , Size : SIZE_T , NewProtection : ULONG , OldProtection : PULONG ,) -> BOOL ; pub fn OpenFileMappingFromApp (DesiredAccess : ULONG , InheritHandle : BOOL , Name : PCWSTR ,) -> HANDLE ; }
};
}
