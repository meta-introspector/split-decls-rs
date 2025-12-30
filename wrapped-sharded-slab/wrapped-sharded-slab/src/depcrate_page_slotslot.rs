// Generated macro for Slot (struct)
macro_rules! Depcrate_page_slotSlot {
() => {
// Module: crate::page::slot
// Provides: {"Slot"}
// Dependencies: {}
pub (crate) struct Slot < T , C > { lifecycle : AtomicUsize , # [doc = " The offset of the next item on the free list."] next : UnsafeCell < usize > , # [doc = " The data stored in the slot."] item : UnsafeCell < T > , _cfg : PhantomData < fn (C) > , }
};
}
