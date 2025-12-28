macro_rules! deps {
    () => {
        Spawn!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T : Unpin > ops :: DerefMut for Spawn < T > { fn deref_mut (& mut self) -> & mut T { & mut self . future } }
    };
}

impl_44!();