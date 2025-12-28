macro_rules! deps {
    () => {
        AsyncSeek!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < T : ? Sized + AsyncSeek + Unpin > AsyncSeek for Box < T > { deref_async_seek ! () ; }
    };
}

impl_112!();