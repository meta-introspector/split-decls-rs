macro_rules! deps {
    () => {
        AsyncRead!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < T : ? Sized + AsyncRead + Unpin > AsyncRead for Box < T > { deref_async_read ! () ; }
    };
}

impl_104!();