macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T > Entry < T > { fn get_value_cell (& self) -> Option < & UnsafeCell < MaybeUninit < T > > > { self . present . load (Ordering :: Acquire) . then_some (& self . value) } # [doc = " # Safety"] # [doc = " The caller must guarantee that there are no concurent mutable accesses into"] # [doc = " this entry's value."] unsafe fn as_ref < 'a > (& self) -> Option < & 'a T > { self . get_value_cell () . map (| cell | unsafe { (& * cell . get ()) . assume_init_ref () }) } }
    };
}

impl_5!()