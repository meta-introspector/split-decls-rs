macro_rules! deps {
    () => {
        SpooledData!();
        SpooledTempFile!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl Write for SpooledTempFile { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { if matches ! { & self . inner , SpooledData :: InMemory (cursor) if cursor . position () . saturating_add (buf . len () as u64) > self . max_size as u64 } { self . roll () ? ; } match & mut self . inner { SpooledData :: InMemory (cursor) => cursor . write (buf) , SpooledData :: OnDisk (file) => file . write (buf) , } } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { if matches ! { & self . inner , SpooledData :: InMemory (cursor) if bufs . iter () . fold (cursor . position () , | a , b | a . saturating_add (b . len () as u64)) > self . max_size as u64 } { self . roll () ? ; } match & mut self . inner { SpooledData :: InMemory (cursor) => cursor . write_vectored (bufs) , SpooledData :: OnDisk (file) => file . write_vectored (bufs) , } } # [inline] fn flush (& mut self) -> io :: Result < () > { match & mut self . inner { SpooledData :: InMemory (cursor) => cursor . flush () , SpooledData :: OnDisk (file) => file . flush () , } } }
    };
}

impl_71!()