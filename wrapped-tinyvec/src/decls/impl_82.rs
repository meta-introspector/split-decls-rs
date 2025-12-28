macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < 's , T > Deref for SliceVec < 's , T > { type Target = [T] ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . data [.. self . len] } }
    };
}

impl_82!()