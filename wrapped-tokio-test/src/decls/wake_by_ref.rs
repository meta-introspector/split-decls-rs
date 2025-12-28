macro_rules! wake_by_ref {
    () => {
        unsafe fn wake_by_ref (raw : * const ()) { let waker = from_raw (raw) ; waker . wake () ; mem :: forget (waker) ; }
    };
}

wake_by_ref!();