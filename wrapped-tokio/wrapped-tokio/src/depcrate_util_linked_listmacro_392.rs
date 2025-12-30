// Generated macro for macro_392 (macro)
macro_rules! Depcrate_util_linked_listmacro_392 {
() => {
// Module: crate::util::linked_list
// Provides: {"macro_392"}
// Dependencies: {}
cfg_taskdump ! { impl < T : Link > LinkedList < T , T :: Target > { pub (crate) fn for_each < F > (& mut self , mut f : F) where F : FnMut (& T :: Handle) , { let mut next = self . head ; while let Some (curr) = next { unsafe { let handle = ManuallyDrop :: new (T :: from_raw (curr)) ; f (& handle) ; next = T :: pointers (curr) . as_ref () . get_next () ; } } } } }
};
}
