macro_rules! deps {
    () => {
        LatchRef!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < L > Deref for LatchRef < '_ , L > { type Target = L ; fn deref (& self) -> & L { unsafe { & * self . inner } } }
    };
}

impl_95!();