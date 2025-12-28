macro_rules! deps {
    () => {
        Message!();
        FlushGuard!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Drop for FlushGuard { fn drop (& mut self) { if let Some (handle) = self . handle . take () { let _ignored = self . sender . send (Message :: Drop) ; if handle . join () . is_err () { eprintln ! ("tracing_chrome: Trace writing thread panicked.") ; } } } }
    };
}

impl_9!()