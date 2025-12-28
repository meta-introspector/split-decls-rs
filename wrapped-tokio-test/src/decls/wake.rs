macro_rules! wake {
    () => {
        unsafe fn wake (raw : * const ()) { let waker = from_raw (raw) ; waker . wake () ; }
    };
}

wake!()