macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < 's , T > DerefMut for SliceVec < 's , T > { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . data [.. self . len] } }
    };
}

impl_83!();