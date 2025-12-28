macro_rules! deps {
    () => {
        LinkedList!();
        Link!();
    };
}

macro_rules! macro_309 {
    () => {
        deps!();
        cfg_taskdump ! { impl < T : Link > LinkedList < T , T :: Target > { pub (crate) fn for_each < F > (& mut self , mut f : F) where F : FnMut (& T :: Handle) , { let mut next = self . head ; while let Some (curr) = next { unsafe { let handle = ManuallyDrop :: new (T :: from_raw (curr)) ; f (& handle) ; next = T :: pointers (curr) . as_ref () . get_next () ; } } } } }
    };
}

macro_309!();