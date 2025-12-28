macro_rules! Registry {
    () => {
        struct Registry { next : AtomicUsize , free : Mutex < VecDeque < usize > > , }
    };
}

Registry!()