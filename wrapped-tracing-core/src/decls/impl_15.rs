macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > Deref for MutexGuard < 'a , T > { type Target = T ; fn deref < 'b > (& 'b self) -> & 'b T { & * self . data } }
    };
}

impl_15!();