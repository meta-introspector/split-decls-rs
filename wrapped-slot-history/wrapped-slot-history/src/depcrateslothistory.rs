// Generated macro for SlotHistory (struct)
macro_rules! DepcrateSlotHistory {
() => {
// Module: crate
// Provides: {"SlotHistory"}
// Dependencies: {}
# [doc = " A bitvector indicating which slots are present in the past epoch."] # [repr (C)] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Clone , PartialEq , Eq)] pub struct SlotHistory { pub bits : BitVec < u64 > , pub next_slot : u64 , }
};
}
