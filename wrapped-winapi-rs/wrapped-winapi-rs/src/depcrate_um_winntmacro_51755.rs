// Generated macro for macro_51755 (macro)
macro_rules! Depcrate_um_winntmacro_51755 {
() => {
// Module: crate::um::winnt
// Provides: {"macro_51755"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] IFDEF ! { STRUCT ! { struct SLIST_HEADER_s { Alignment : ULONGLONG , Region : ULONGLONG , } } STRUCT ! { struct SLIST_HEADER_HeaderX64 { BitFields1 : ULONGLONG , BitFields2 : ULONGLONG , } } BITFIELD ! { SLIST_HEADER_HeaderX64 BitFields1 : ULONGLONG [Depth set_Depth [0 .. 16] , Sequence set_Sequence [16 .. 64] ,] } BITFIELD ! { SLIST_HEADER_HeaderX64 BitFields2 : ULONGLONG [Reserved set_Reserved [0 .. 4] , NextEntry set_NextEntry [4 .. 64] ,] } UNION ! { union SLIST_HEADER { [u64 ; 2] , s s_mut : SLIST_HEADER_s , HeaderX64 HeaderX64_mut : SLIST_HEADER_HeaderX64 , } } pub type PSLIST_HEADER = * mut SLIST_HEADER ; }
};
}
