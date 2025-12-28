macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Drop for Stream { fn drop (& mut self) { unsafe { lzma_sys :: lzma_end (& mut self . raw) ; } } }
    };
}

impl_27!();