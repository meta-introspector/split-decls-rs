macro_rules! deps {
    () => {
        Tid!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < C > Tid < C > { # [cold] fn poisoned () -> Self { Self { id : std :: usize :: MAX , _not_send : PhantomData , _cfg : PhantomData , } } # [doc = " Returns true if the local thread ID was accessed while unwinding."] pub (crate) fn is_poisoned (& self) -> bool { self . id == std :: usize :: MAX } }
    };
}

impl_168!()