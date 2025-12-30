// Generated macro for impl_320 (impl)
macro_rules! Depcrate_internedimpl_320 {
() => {
// Module: crate::interned
// Provides: {"impl_320"}
// Dependencies: {}
impl < C > Value < C > where C : Configuration , { # [doc = " Fields of this interned struct."] # [cfg (feature = "salsa_unstable")] pub fn fields (& self) -> & C :: Fields < 'static > { unsafe { & * self . fields . get () } } # [doc = " Returns memory usage information about the interned value."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `MemoTable` must belong to a `Value` of the correct type. Additionally, the"] # [doc = " lock must be held for the shard containing the value."] # [cfg (all (not (feature = "shuttle") , feature = "salsa_unstable"))] unsafe fn memory_usage (& self , memo_table_types : & MemoTableTypes) -> crate :: database :: SlotInfo { let heap_size = C :: heap_size (self . fields ()) ; let memos = unsafe { & * self . memos . get () } ; let memos = unsafe { memo_table_types . attach_memos (memos) } ; crate :: database :: SlotInfo { debug_name : C :: DEBUG_NAME , size_of_metadata : std :: mem :: size_of :: < Self > () - std :: mem :: size_of :: < C :: Fields < '_ > > () , size_of_fields : std :: mem :: size_of :: < C :: Fields < '_ > > () , heap_size_of_fields : heap_size , memos : memos . memory_usage () , } } }
};
}
