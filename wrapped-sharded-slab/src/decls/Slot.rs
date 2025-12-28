macro_rules! Slot {
    () => {
        pub (crate) struct Slot < T , C > { lifecycle : AtomicUsize , # [doc = " The offset of the next item on the free list."] next : UnsafeCell < usize > , # [doc = " The data stored in the slot."] item : UnsafeCell < T > , _cfg : PhantomData < fn (C) > , }
    };
}

Slot!()