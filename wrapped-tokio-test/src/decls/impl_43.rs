macro_rules! deps {
    () => {
        Spawn!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T : Unpin > ops :: Deref for Spawn < T > { type Target = T ; fn deref (& self) -> & T { & self . future } }
    };
}

impl_43!()