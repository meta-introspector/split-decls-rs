macro_rules! deps {
    () => {
        Local!();
        TransferStack!();
        Slot!();
        Addr!();
        Shared!();
        FreeList!();
        Config!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < T , C > Shared < T , C > where C : cfg :: Config , { const NULL : usize = Addr :: < C > :: NULL ; pub (crate) fn new (size : usize , prev_sz : usize) -> Self { Self { prev_sz , size , remote : stack :: TransferStack :: new () , slab : UnsafeCell :: new (None) , } } # [doc = " Return the head of the freelist"] # [doc = ""] # [doc = " If there is space on the local list, it returns the head of the local list. Otherwise, it"] # [doc = " pops all the slots from the global list and returns the head of that list"] # [doc = ""] # [doc = " *Note*: The local list's head is reset when setting the new state in the slot pointed to be"] # [doc = " `head` returned from this function"] # [inline] fn pop (& self , local : & Local) -> Option < usize > { let head = local . head () ; test_println ! ("-> local head {:?}" , head) ; let head = if head < self . size { head } else { let head = self . remote . pop_all () ; test_println ! ("-> remote head {:?}" , head) ; head ? } ; if head == Self :: NULL { test_println ! ("-> NULL! {:?}" , head) ; None } else { Some (head) } } # [doc = " Returns `true` if storage is currently allocated for this page, `false`"] # [doc = " otherwise."] # [inline] fn is_unallocated (& self) -> bool { self . slab . with (| s | unsafe { (* s) . is_none () }) } # [inline] pub (crate) fn with_slot < 'a , U > (& 'a self , addr : Addr < C > , f : impl FnOnce (& 'a Slot < T , C >) -> Option < U > ,) -> Option < U > { let poff = addr . offset () - self . prev_sz ; test_println ! ("-> offset {:?}" , poff) ; self . slab . with (| slab | { let slot = unsafe { & * slab } . as_ref () ? . get (poff) ? ; f (slot) }) } # [inline (always)] pub (crate) fn free_list (& self) -> & impl FreeList < C > { & self . remote } }
    };
}

impl_132!();