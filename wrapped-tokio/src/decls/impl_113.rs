macro_rules! deps {
    () => {
        AsyncSeek!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < T : ? Sized + AsyncSeek + Unpin > AsyncSeek for & mut T { deref_async_seek ! () ; }
    };
}

impl_113!();