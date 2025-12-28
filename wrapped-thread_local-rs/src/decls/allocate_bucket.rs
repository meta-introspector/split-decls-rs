macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! allocate_bucket {
    () => {
        deps!();
        fn allocate_bucket < T > (size : usize) -> * mut Entry < T > { Box :: into_raw ((0 .. size) . map (| _ | Entry :: < T > { present : AtomicBool :: new (false) , value : UnsafeCell :: new (MaybeUninit :: uninit ()) , }) . collect () ,) as * mut _ }
    };
}

allocate_bucket!();