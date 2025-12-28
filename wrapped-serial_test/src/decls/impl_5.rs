macro_rules! deps {
    () => {
        UniqueReentrantMutex!();
        Locks!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl UniqueReentrantMutex { fn new_mutex (name : & str) -> Self { Self { locks : Locks :: new (name) , id : MUTEX_ID . fetch_add (1 , std :: sync :: atomic :: Ordering :: SeqCst) , } } }
    };
}

impl_5!();