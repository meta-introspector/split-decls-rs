// Generated macro for other_51757 (other)
macro_rules! Depcrate_um_winntother_51757 {
() => {
// Module: crate::um::winnt
// Provides: {"other_51757"}
// Dependencies: {}
extern "system" { pub fn RtlInitializeSListHead (ListHead : PSLIST_HEADER ,) ; pub fn RtlFirstEntrySList (ListHead : * const SLIST_HEADER ,) -> PSLIST_ENTRY ; pub fn RtlInterlockedPopEntrySList (ListHead : PSLIST_HEADER ,) -> PSLIST_ENTRY ; pub fn RtlInterlockedPushEntrySList (ListHead : PSLIST_HEADER , ListEntry : PSLIST_ENTRY ,) -> PSLIST_ENTRY ; pub fn RtlInterlockedPushListSListEx (ListHead : PSLIST_HEADER , ListEntry : PSLIST_ENTRY , ListEnd : PSLIST_ENTRY , Count : DWORD ,) -> PSLIST_ENTRY ; pub fn RtlInterlockedFlushSList (ListHead : PSLIST_HEADER ,) -> PSLIST_ENTRY ; pub fn RtlQueryDepthSList (ListHead : PSLIST_HEADER ,) -> WORD ; }
};
}
