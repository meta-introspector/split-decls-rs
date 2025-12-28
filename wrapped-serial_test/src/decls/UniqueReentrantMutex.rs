macro_rules! deps {
    () => {
        Locks!();
    };
}

macro_rules! UniqueReentrantMutex {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct UniqueReentrantMutex { locks : Locks , # [allow (dead_code)] pub (crate) id : u32 , }
    };
}

UniqueReentrantMutex!()