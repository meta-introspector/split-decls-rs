macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < T > std :: ops :: Deref for Frozen < T > { type Target = T ; fn deref (& self) -> & T { & self . 0 } }
    };
}

impl_58!()