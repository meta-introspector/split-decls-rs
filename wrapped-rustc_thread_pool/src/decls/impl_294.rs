macro_rules! deps {
    () => {
        WorkerLocal!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl < T > Deref for WorkerLocal < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { self . current () } }
    };
}

impl_294!()