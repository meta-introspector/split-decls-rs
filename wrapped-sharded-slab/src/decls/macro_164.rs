macro_rules! deps {
    () => {
        Registry!();
    };
}

macro_rules! macro_164 {
    () => {
        deps!();
        lazy_static ! { static ref REGISTRY : Registry = Registry { next : AtomicUsize :: new (0) , free : Mutex :: new (VecDeque :: new ()) , } ; }
    };
}

macro_164!()