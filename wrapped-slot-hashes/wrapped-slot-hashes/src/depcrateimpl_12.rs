// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl SlotHashes { pub fn add (& mut self , slot : u64 , hash : Hash) { match self . binary_search_by (| (probe , _) | slot . cmp (probe)) { Ok (index) => (self . 0) [index] = (slot , hash) , Err (index) => (self . 0) . insert (index , (slot , hash)) , } (self . 0) . truncate (get_entries ()) ; } pub fn position (& self , slot : & u64) -> Option < usize > { self . binary_search_by (| (probe , _) | slot . cmp (probe)) . ok () } # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn get (& self , slot : & u64) -> Option < & Hash > { self . binary_search_by (| (probe , _) | slot . cmp (probe)) . ok () . map (| index | & self [index] . 1) } pub fn new (slot_hashes : & [SlotHash]) -> Self { let mut slot_hashes = slot_hashes . to_vec () ; slot_hashes . sort_by (| (a , _) , (b , _) | b . cmp (a)) ; Self (slot_hashes) } pub fn slot_hashes (& self) -> & [SlotHash] { & self . 0 } }
};
}
