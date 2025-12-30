// Generated macro for TrieBuilderStore (trait)
macro_rules! Depcrate_builder_nonconst_storeTrieBuilderStore {
() => {
// Module: crate::builder::nonconst::store
// Provides: {"TrieBuilderStore"}
// Dependencies: {}
# [doc = " A trait applied to a data structure for building a ZeroTrie."] pub (crate) trait TrieBuilderStore { # [doc = " Create a new empty store."] fn atbs_new_empty () -> Self ; # [doc = " Return the length in bytes of the store."] fn atbs_len (& self) -> usize ; # [doc = " Push a byte to the front of the store."] fn atbs_push_front (& mut self , byte : u8) ; # [doc = " Push multiple bytes to the front of the store."] fn atbs_extend_front (& mut self , other : & [u8]) ; # [doc = " Read the store into a `Vec<u8>`."] fn atbs_to_bytes (& self) -> Vec < u8 > ; # [doc = " Perform the operation `self[index] |= bits`"] fn atbs_bitor_assign (& mut self , index : usize , bits : u8) ; # [doc = " Swap the adjacent ranges `self[start..mid]` and `self[mid..limit]`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the specified ranges are invalid."] fn atbs_swap_ranges (& mut self , start : usize , mid : usize , limit : usize) ; # [doc = " Remove and return the first element in the store, or `None` if empty."] fn atbs_pop_front (& mut self) -> Option < u8 > ; # [doc = " Prepend `n` zeros to the front of the store."] fn atbs_prepend_n_zeros (& mut self , n : usize) { let mut i = 0 ; while i < n { self . atbs_push_front (0) ; i += 1 ; } } }
};
}
