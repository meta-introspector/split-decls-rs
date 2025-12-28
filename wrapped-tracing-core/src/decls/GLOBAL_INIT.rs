macro_rules! GLOBAL_INIT {
    () => {
        static GLOBAL_INIT : AtomicUsize = AtomicUsize :: new (UNINITIALIZED) ;
    };
}

GLOBAL_INIT!()