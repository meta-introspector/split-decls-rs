macro_rules! drop_waker {
    () => {
        unsafe fn drop_waker (raw : * const ()) { let _ = from_raw (raw) ; }
    };
}

drop_waker!();