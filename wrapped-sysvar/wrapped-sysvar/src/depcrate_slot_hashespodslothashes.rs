// Generated macro for PodSlotHashes (struct)
macro_rules! Depcrate_slot_hashesPodSlotHashes {
() => {
// Module: crate::slot_hashes
// Provides: {"PodSlotHashes"}
// Dependencies: {}
# [cfg (feature = "bytemuck")] # [doc = " API for querying of the `SlotHashes` sysvar by on-chain programs."] # [doc = ""] # [doc = " Hangs onto the allocated raw buffer from the account data, which can be"] # [doc = " queried or accessed directly as a slice of `PodSlotHash`."] # [derive (Default)] pub struct PodSlotHashes { data : Vec < u8 > , slot_hashes_start : usize , slot_hashes_end : usize , }
};
}
