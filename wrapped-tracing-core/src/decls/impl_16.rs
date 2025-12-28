macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > DerefMut for MutexGuard < 'a , T > { fn deref_mut < 'b > (& 'b mut self) -> & 'b mut T { & mut * self . data } }
    };
}

impl_16!();