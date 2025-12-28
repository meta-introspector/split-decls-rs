macro_rules! deps {
    () => {
        Finish!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'a > Drop for Finish < 'a > { fn drop (& mut self) { if self . panicked { self . state . store (PANICKED , Ordering :: SeqCst) ; } } }
    };
}

impl_29!()