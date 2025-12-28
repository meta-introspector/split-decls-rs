macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T > Mutex < T > { # [doc = " Creates a new spinlock wrapping the supplied data."] pub (crate) const fn new (user_data : T) -> Mutex < T > { Mutex { lock : AtomicBool :: new (false) , data : UnsafeCell :: new (user_data) , } } }
    };
}

impl_11!()