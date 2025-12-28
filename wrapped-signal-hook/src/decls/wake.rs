macro_rules! deps {
    () => {
        WakeMethod!();
    };
}

macro_rules! wake {
    () => {
        deps!();
        pub (crate) fn wake (pipe : RawFd , method : WakeMethod) { unsafe { let data = b"X" as * const _ as * const _ ; match method { WakeMethod :: Write => libc :: write (pipe , data , 1) , WakeMethod :: Send => libc :: send (pipe , data , 1 , MSG_NOWAIT) , } ; } }
    };
}

wake!();