macro_rules! deps {
    () => {
        ZipFileSeekReader!();
        ZipFileSeek!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < R : Seek > Seek for ZipFileSeek < '_ , R > { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { match & mut self . reader { ZipFileSeekReader :: Raw (r) => r . seek (pos) , } } }
    };
}

impl_120!();