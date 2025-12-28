macro_rules! deps {
    () => {
        FromDyn!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl < T > std :: ops :: DerefMut for FromDyn < T > { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_287!()