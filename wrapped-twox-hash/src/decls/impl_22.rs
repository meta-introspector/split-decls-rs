macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl hash :: Hasher for Hasher { # [inline] fn write (& mut self , data : & [u8]) { let len = data . len () ; let (buffered_lanes , data) = self . buffer . extend (data) ; if let Some (& lanes) = buffered_lanes { self . accumulators . write (lanes) ; } let data = self . accumulators . write_many (data) ; self . buffer . set (data) ; self . length += len . into_u64 () ; } # [inline] fn finish (& self) -> u64 { Hasher :: finish_32 (self) . into () } }
    };
}

impl_22!()