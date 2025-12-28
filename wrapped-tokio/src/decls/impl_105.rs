macro_rules! deps {
    () => {
        AsyncRead!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < T : ? Sized + AsyncRead + Unpin > AsyncRead for & mut T { deref_async_read ! () ; }
    };
}

impl_105!();