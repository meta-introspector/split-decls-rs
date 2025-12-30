// Generated macro for make_id (function)
macro_rules! Depcrate_tablemake_id {
() => {
// Module: crate::table
// Provides: {"make_id"}
// Dependencies: {}
fn make_id (page : PageIndex , slot : SlotIndex) -> Id { let page = page . 0 as u32 ; let slot = slot . 0 as u32 ; unsafe { Id :: from_index ((page << PAGE_LEN_BITS) | slot) } }
};
}
