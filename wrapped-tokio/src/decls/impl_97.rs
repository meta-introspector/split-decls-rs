macro_rules! deps {
    () => {
        AsyncBufRead!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < T : ? Sized + AsyncBufRead + Unpin > AsyncBufRead for & mut T { deref_async_buf_read ! () ; }
    };
}

impl_97!()