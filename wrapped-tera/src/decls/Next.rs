macro_rules! Next {
    () => {
        struct Next (AtomicUsize) ;
    };
}

Next!();