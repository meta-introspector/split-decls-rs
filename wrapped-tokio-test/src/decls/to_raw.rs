macro_rules! deps {
    () => {
        ThreadWaker!();
    };
}

macro_rules! to_raw {
    () => {
        deps!();
        unsafe fn to_raw (waker : Arc < ThreadWaker >) -> RawWaker { RawWaker :: new (Arc :: into_raw (waker) as * const () , & VTABLE) }
    };
}

to_raw!()