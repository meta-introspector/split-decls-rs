macro_rules! RecursiveGuard {
    () => {
        struct RecursiveGuard (& 'static LocalKey < AtomicBool >) ;
    };
}

RecursiveGuard!()