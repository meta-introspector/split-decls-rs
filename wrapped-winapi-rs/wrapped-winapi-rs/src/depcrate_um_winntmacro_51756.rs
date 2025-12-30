// Generated macro for macro_51756 (macro)
macro_rules! Depcrate_um_winntmacro_51756 {
() => {
// Module: crate::um::winnt
// Provides: {"macro_51756"}
// Dependencies: {}
# [cfg (target_pointer_width = "32")] IFDEF ! { STRUCT ! { struct SLIST_HEADER_s { Next : SLIST_ENTRY , Depth : WORD , Reserved : WORD , } } UNION ! { union SLIST_HEADER { [u64 ; 1] , Alignment Alignment_mut : ULONGLONG , s s_mut : SLIST_HEADER_s , } } pub type PSLIST_HEADER = * mut SLIST_HEADER ; }
};
}
