macro_rules! deps {
    () => {
        SpooledTempFile!();
        SpooledData!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl Seek for SpooledTempFile { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { match & mut self . inner { SpooledData :: InMemory (cursor) => cursor . seek (pos) , SpooledData :: OnDisk (file) => file . seek (pos) , } } }
    };
}

impl_72!()