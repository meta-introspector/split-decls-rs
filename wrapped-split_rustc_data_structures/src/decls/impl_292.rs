macro_rules! deps {
    () => {
        IntoDynSyncSend!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl < T > std :: ops :: DerefMut for IntoDynSyncSend < T > { # [inline (always)] fn deref_mut (& mut self) -> & mut T { & mut self . 0 } }
    };
}

impl_292!()