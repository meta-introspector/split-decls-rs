macro_rules! deps {
    () => {
        ZipFileSeek!();
        ZipFileSeekReader!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < R : Read > Read for ZipFileSeek < '_ , R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { match & mut self . reader { ZipFileSeekReader :: Raw (r) => r . read (buf) , } } }
    };
}

impl_119!();