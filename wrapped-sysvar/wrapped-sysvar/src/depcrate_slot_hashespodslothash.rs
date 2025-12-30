// Generated macro for PodSlotHash (struct)
macro_rules! Depcrate_slot_hashesPodSlotHash {
() => {
// Module: crate::slot_hashes
// Provides: {"PodSlotHash"}
// Dependencies: {}
# [doc = " A bytemuck-compatible (plain old data) version of `SlotHash`."] # [cfg_attr (feature = "bytemuck" , derive (Pod , Zeroable))] # [derive (Copy , Clone , Default)] # [repr (C)] pub struct PodSlotHash { pub slot : Slot , pub hash : Hash , }
};
}
