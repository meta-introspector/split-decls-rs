// Generated macro for Map (struct)
macro_rules! Depcrate_dataMap {
() => {
// Module: crate::data
// Provides: {"Map"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , Default)] # [repr (C)] pub struct Map { # [doc = " The offset inside the file that is being mapped."] pub offset : usize , # [doc = " The size of the memory map."] pub size : usize , # [doc = " Contains both prot and map flags."] pub flags : MapFlags , # [doc = " Functions as a hint to where in the virtual address space of the running process, to place"] # [doc = " the memory map. If [`MapFlags::MAP_FIXED`] is set, then this address must be the address to"] # [doc = " map to."] pub address : usize , }
};
}
