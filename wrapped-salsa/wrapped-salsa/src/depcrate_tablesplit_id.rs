// Generated macro for split_id (function)
macro_rules! Depcrate_tablesplit_id {
() => {
// Module: crate::table
// Provides: {"split_id"}
// Dependencies: {}
# [inline] pub fn split_id (id : Id) -> (PageIndex , SlotIndex) { let index = id . index () as usize ; let slot = index & PAGE_LEN_MASK ; let page = index >> PAGE_LEN_BITS ; (PageIndex :: new (page) , SlotIndex :: new (slot)) }
};
}
