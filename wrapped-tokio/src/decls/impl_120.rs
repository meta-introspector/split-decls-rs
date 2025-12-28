macro_rules! deps {
    () => {
        AsyncWrite!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < T : ? Sized + AsyncWrite + Unpin > AsyncWrite for & mut T { deref_async_write ! () ; }
    };
}

impl_120!()