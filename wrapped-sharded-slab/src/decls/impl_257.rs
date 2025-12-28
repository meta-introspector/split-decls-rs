macro_rules! deps {
    () => {
        SetDropped!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl Drop for SetDropped { fn drop (& mut self) { self . dropped . store (true , Ordering :: SeqCst) ; } }
    };
}

impl_257!()