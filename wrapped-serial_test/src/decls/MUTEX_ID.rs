macro_rules! MUTEX_ID {
    () => {
        static MUTEX_ID : AtomicU32 = AtomicU32 :: new (1) ;
    };
}

MUTEX_ID!();