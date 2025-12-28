macro_rules! deps {
    () => {
        Clear!();
        Slot!();
        Lifecycle!();
        FreeList!();
        Generation!();
        Config!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < T , C > Slot < T , C > where T : Default + Clear , C : cfg :: Config , { pub (in crate :: page) fn new (next : usize) -> Self { Self { lifecycle : AtomicUsize :: new (Lifecycle :: < C > :: REMOVING . as_usize ()) , item : UnsafeCell :: new (T :: default ()) , next : UnsafeCell :: new (next) , _cfg : PhantomData , } } # [doc = " Try to clear this slot's storage"] # [doc = ""] # [doc = " If there are references to this slot, then we mark this slot for clearing and let the last"] # [doc = " thread do the work for us."] # [inline] pub (super) fn try_clear_storage < F : FreeList < C > > (& self , gen : Generation < C > , offset : usize , free : & F ,) -> bool { let should_clear = match self . mark_release (gen) { Some (should_clear) => should_clear , None => { test_println ! ("-> try_clear_storage; nothing exists at generation={:?}" , gen) ; return false ; } } ; test_println ! ("-> try_clear_storage; marked!") ; if should_clear { test_println ! ("-> try_remove_value; can clear now") ; return self . clear_storage (gen , offset , free) ; } true } # [doc = " Clear this slot's storage"] # [doc = ""] # [doc = " This method blocks until all references have been dropped and clears the storage."] pub (super) fn clear_storage < F : FreeList < C > > (& self , gen : Generation < C > , offset : usize , free : & F ,) -> bool { self . release_with (gen , offset , free , | item | { let cleared = item . map (| inner | Clear :: clear (inner)) . is_some () ; test_println ! ("-> cleared: {}" , cleared) ; cleared }) } }
    };
}

impl_86!();