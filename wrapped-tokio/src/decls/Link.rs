macro_rules! deps {
    () => {
        Pointers!();
    };
}

macro_rules! Link {
    () => {
        deps!();
        # [doc = " Defines how a type is tracked within a linked list."] # [doc = ""] # [doc = " In order to support storing a single type within multiple lists, accessing"] # [doc = " the list pointers is decoupled from the entry type."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementations must guarantee that `Target` types are pinned in memory. In"] # [doc = " other words, when a node is inserted, the value will not be moved as long as"] # [doc = " it is stored in the list."] pub (crate) unsafe trait Link { # [doc = " Handle to the list entry."] # [doc = ""] # [doc = " This is usually a pointer-ish type."] type Handle ; # [doc = " Node type."] type Target ; # [doc = " Convert the handle to a raw pointer without consuming the handle."] # [allow (clippy :: wrong_self_convention)] fn as_raw (handle : & Self :: Handle) -> NonNull < Self :: Target > ; # [doc = " Convert the raw pointer to a handle"] unsafe fn from_raw (ptr : NonNull < Self :: Target >) -> Self :: Handle ; # [doc = " Return the pointers for a node"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The resulting pointer should have the same tag in the stacked-borrows"] # [doc = " stack as the argument. In particular, the method may not create an"] # [doc = " intermediate reference in the process of creating the resulting raw"] # [doc = " pointer."] # [doc = ""] # [doc = " The `target` pointer must be valid."] unsafe fn pointers (target : NonNull < Self :: Target >) -> NonNull < Pointers < Self :: Target > > ; }
    };
}

Link!();