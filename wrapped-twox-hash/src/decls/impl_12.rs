macro_rules! deps {
    () => {
        Lane!();
        BufferData!();
        Bytes!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl BufferData { const fn new () -> Self { Self ([0 ; 4]) } const fn bytes (& self) -> & Bytes { const _ : () = assert ! (mem :: align_of ::< u8 > () <= mem :: align_of ::< Lane > ()) ; unsafe { & * self . 0 . as_ptr () . cast () } } fn bytes_mut (& mut self) -> & mut Bytes { unsafe { & mut * self . 0 . as_mut_ptr () . cast () } } }
    };
}

impl_12!()