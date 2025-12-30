// Generated macro for impl_117 (impl)
macro_rules! Depcrate_page_slotimpl_117 {
() => {
// Module: crate::page::slot
// Provides: {"impl_117"}
// Dependencies: {}
impl < T , C : cfg :: Config > Guard < T , C > { # [doc = " Releases the guard, returning `true` if the slot should be cleared."] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " This dereferences a raw pointer to the slot. The caller is responsible"] # [doc = " for ensuring that the `Guard` does not outlive the slab that contains"] # [doc = " the pointed slot. Failure to do so means this pointer may dangle."] # [inline] pub (crate) unsafe fn release (& self) -> bool { self . slot () . release () } # [doc = " Returns a borrowed reference to the slot."] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " This dereferences a raw pointer to the slot. The caller is responsible"] # [doc = " for ensuring that the `Guard` does not outlive the slab that contains"] # [doc = " the pointed slot. Failure to do so means this pointer may dangle."] # [inline] pub (crate) unsafe fn slot (& self) -> & Slot < T , C > { self . slot . as_ref () } # [doc = " Returns a borrowed reference to the slot's value."] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " This dereferences a raw pointer to the slot. The caller is responsible"] # [doc = " for ensuring that the `Guard` does not outlive the slab that contains"] # [doc = " the pointed slot. Failure to do so means this pointer may dangle."] # [inline (always)] pub (crate) unsafe fn value (& self) -> & T { self . slot () . item . with (| item | & * item) } }
};
}
