// Generated macro for Slot (trait)
macro_rules! Depcrate_tableSlot {
() => {
// Module: crate::table
// Provides: {"Slot"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Implementors of this trait need to make sure that their type is unique with respect to"] # [doc = " their owning ingredient as the allocation strategy relies on this."] pub unsafe trait Slot : Any + Send + Sync { # [doc = " Access the [`MemoTable`][] for this slot."] # [doc = ""] # [doc = " # Safety condition"] # [doc = ""] # [doc = " The current revision MUST be the current revision of the database containing this slot."] unsafe fn memos (& self , current_revision : Revision) -> & MemoTable ; # [doc = " Mutably access the [`MemoTable`] for this slot."] fn memos_mut (& mut self) -> & mut MemoTable ; }
};
}
