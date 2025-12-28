macro_rules! INTERNAL_COUNTER {
    () => {
        # [doc (hidden)] pub static INTERNAL_COUNTER : AtomicU32 = AtomicU32 :: new (0) ;
    };
}

INTERNAL_COUNTER!()