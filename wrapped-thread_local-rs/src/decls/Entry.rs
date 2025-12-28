macro_rules! Entry {
    () => {
        struct Entry < T > { present : AtomicBool , value : UnsafeCell < MaybeUninit < T > > , }
    };
}

Entry!();