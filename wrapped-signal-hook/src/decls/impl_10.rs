macro_rules! deps {
    () => {
        DeliveryState!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl DeliveryState { fn new () -> Self { let ids = (0 .. MAX_SIGNUM) . map (| _ | None) . collect () ; Self { closed : AtomicBool :: new (false) , registered_signal_ids : Mutex :: new (ids) , } } }
    };
}

impl_10!();