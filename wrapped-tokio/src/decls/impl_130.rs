macro_rules! deps {
    () => {
        ReadBuf!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        # [cfg (feature = "io-util")] # [cfg_attr (docsrs , doc (cfg (feature = "io-util")))] unsafe impl < 'a > bytes :: BufMut for ReadBuf < 'a > { fn remaining_mut (& self) -> usize { self . remaining () } unsafe fn advance_mut (& mut self , cnt : usize) { unsafe { self . assume_init (cnt) ; } self . advance (cnt) ; } fn chunk_mut (& mut self) -> & mut bytes :: buf :: UninitSlice { let unfilled = unsafe { self . unfilled_mut () } ; let len = unfilled . len () ; let ptr = unfilled . as_mut_ptr () as * mut u8 ; unsafe { bytes :: buf :: UninitSlice :: from_raw_parts_mut (ptr , len) } } }
    };
}

impl_130!();