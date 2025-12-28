macro_rules! deps {
    () => {
        KernelSigSet!();
        Signal!();
        Result!();
    };
}

macro_rules! impl_1648 {
    () => {
        deps!();
        impl fmt :: Debug for KernelSigSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_set () ; for i in 1 ..= _NSIG { let sig = unsafe { Signal :: from_raw_unchecked (i as _) } ; if self . contains (sig) { d . entry (& sig) ; } } d . finish () } }
    };
}

impl_1648!();