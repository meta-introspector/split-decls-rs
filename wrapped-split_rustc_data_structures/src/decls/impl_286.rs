macro_rules! deps {
    () => {
        FromDyn!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl < T > std :: ops :: Deref for FromDyn < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_286!();