macro_rules! deps {
    () => {
        ZipFileSeekReader!();
        ZipFileSeek!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < R : Read > Read for ZipFileSeek < '_ , R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { match & mut self . reader { ZipFileSeekReader :: Raw (r) => r . read (buf) , } } }
    };
}

impl_119!()