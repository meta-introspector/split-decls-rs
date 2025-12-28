macro_rules! deps {
    () => {
        LinkedList!();
        Link!();
    };
}

macro_rules! macro_308 {
    () => {
        deps!();
        cfg_io_driver_impl ! { pub (crate) struct DrainFilter <'a , T : Link , F > { list : &'a mut LinkedList < T , T :: Target >, filter : F , curr : Option < NonNull < T :: Target >>, } impl < T : Link > LinkedList < T , T :: Target > { pub (crate) fn drain_filter < F > (& mut self , filter : F) -> DrainFilter <'_ , T , F > where F : FnMut (& T :: Target) -> bool , { let curr = self . head ; DrainFilter { curr , filter , list : self , } } } impl <'a , T , F > Iterator for DrainFilter <'a , T , F > where T : Link , F : FnMut (& T :: Target) -> bool , { type Item = T :: Handle ; fn next (& mut self) -> Option < Self :: Item > { while let Some (curr) = self . curr { self . curr = unsafe { T :: pointers (curr) . as_ref () } . get_next () ; if (self . filter) (unsafe { & mut * curr . as_ptr () }) { return unsafe { self . list . remove (curr) } ; } } None } } }
    };
}

macro_308!();