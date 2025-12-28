macro_rules! deps {
    () => {
        AsyncWrite!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < T : ? Sized + AsyncWrite + Unpin > AsyncWrite for Box < T > { deref_async_write ! () ; }
    };
}

impl_119!();