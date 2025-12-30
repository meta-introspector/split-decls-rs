// Generated macro for impl_280 (impl)
macro_rules! Depcrate_inputimpl_280 {
() => {
// Module: crate::input
// Provides: {"impl_280"}
// Dependencies: {}
impl < C > Value < C > where C : Configuration , { # [doc = " Fields of this tracked struct."] # [doc = ""] # [doc = " They can change across revisions, but they do not change within"] # [doc = " a particular revision."] # [cfg (feature = "salsa_unstable")] pub fn fields (& self) -> & C :: Fields { & self . fields } # [doc = " Returns memory usage information about the input."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `MemoTable` must belong to a `Value` of the correct type."] # [cfg (feature = "salsa_unstable")] unsafe fn memory_usage (& self , memo_table_types : & MemoTableTypes) -> crate :: database :: SlotInfo { let heap_size = C :: heap_size (& self . fields) ; let memos = unsafe { memo_table_types . attach_memos (& self . memos) } ; crate :: database :: SlotInfo { debug_name : C :: DEBUG_NAME , size_of_metadata : std :: mem :: size_of :: < Self > () - std :: mem :: size_of :: < C :: Fields > () , size_of_fields : std :: mem :: size_of :: < C :: Fields > () , heap_size_of_fields : heap_size , memos : memos . memory_usage () , } } }
};
}
