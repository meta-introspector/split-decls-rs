macro_rules! deps {
    () => {
        Socket!();
        MaybeUninitSlice!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a > Read for & 'a Socket { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let buf = unsafe { & mut * (buf as * mut [u8] as * mut [MaybeUninit < u8 >]) } ; self . recv (buf) } # [cfg (not (target_os = "redox"))] fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { let bufs = unsafe { & mut * (bufs as * mut [IoSliceMut < '_ >] as * mut [MaybeUninitSlice < '_ >]) } ; self . recv_vectored (bufs) . map (| (n , _) | n) } }
    };
}

impl_31!()