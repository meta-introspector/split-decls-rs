macro_rules! clone {
    () => {
        unsafe fn clone (raw : * const ()) -> RawWaker { let waker = from_raw (raw) ; mem :: forget (waker . clone ()) ; to_raw (waker) }
    };
}

clone!();