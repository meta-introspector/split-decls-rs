macro_rules! deps {
    () => {
        Event!();
        EventKind!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl Event { pub fn new (kind : EventKind) -> Self { Self { thread_id : thread :: current () . id () , kind , } } }
    };
}

impl_99!();