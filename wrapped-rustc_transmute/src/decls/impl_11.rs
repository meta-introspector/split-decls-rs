macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl State { pub (crate) fn new () -> Self { static COUNTER : AtomicU32 = AtomicU32 :: new (0) ; Self (COUNTER . fetch_add (1 , Ordering :: SeqCst)) } }
    };
}

impl_11!()