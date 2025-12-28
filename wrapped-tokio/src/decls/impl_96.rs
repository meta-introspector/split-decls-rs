macro_rules! deps {
    () => {
        AsyncBufRead!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < T : ? Sized + AsyncBufRead + Unpin > AsyncBufRead for Box < T > { deref_async_buf_read ! () ; }
    };
}

impl_96!();