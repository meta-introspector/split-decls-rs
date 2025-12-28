macro_rules! deps {
    () => {
        ZipWriterStats!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl ZipWriterStats { fn update (& mut self , buf : & [u8]) { self . hasher . update (buf) ; self . bytes_written += buf . len () as u64 ; } }
    };
}

impl_250!()