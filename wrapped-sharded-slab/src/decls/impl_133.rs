macro_rules! deps {
    () => {
        FreeList!();
        Addr!();
        Shared!();
        Slot!();
        Iter!();
        Generation!();
        Config!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < 'a , T , C > Shared < Option < T > , C > where C : cfg :: Config + 'a , { pub (crate) fn take < F > (& self , addr : Addr < C > , gen : slot :: Generation < C > , free_list : & F ,) -> Option < T > where F : FreeList < C > , { let offset = addr . offset () - self . prev_sz ; test_println ! ("-> take: offset {:?}" , offset) ; self . slab . with (| slab | { let slab = unsafe { & * slab } . as_ref () ? ; let slot = slab . get (offset) ? ; slot . remove_value (gen , offset , free_list) }) } pub (crate) fn remove < F : FreeList < C > > (& self , addr : Addr < C > , gen : slot :: Generation < C > , free_list : & F ,) -> bool { let offset = addr . offset () - self . prev_sz ; test_println ! ("-> offset {:?}" , offset) ; self . slab . with (| slab | { let slab = unsafe { & * slab } . as_ref () ; if let Some (slot) = slab . and_then (| slab | slab . get (offset)) { slot . try_remove_value (gen , offset , free_list) } else { false } }) } fn make_ref (slot : & 'a Slot < Option < T > , C >) -> Option < & 'a T > { slot . value () . as_ref () } pub (crate) fn iter (& self) -> Option < Iter < 'a , T , C > > { let slab = self . slab . with (| slab | unsafe { (* slab) . as_ref () }) ; slab . map (| slab | { slab . iter () . filter_map (Shared :: make_ref as fn (& 'a Slot < Option < T > , C >) -> Option < & 'a T >) }) } }
    };
}

impl_133!()