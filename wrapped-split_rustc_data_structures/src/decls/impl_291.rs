macro_rules! deps {
    () => {
        IntoDynSyncSend!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < T > std :: ops :: Deref for IntoDynSyncSend < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { & self . 0 } }
    };
}

impl_291!()