macro_rules! deps {
    () => {
        SeekableTake!();
    };
}

macro_rules! ZipFileSeekReader {
    () => {
        deps!();
        enum ZipFileSeekReader < 'a , R > { Raw (SeekableTake < 'a , R >) , }
    };
}

ZipFileSeekReader!()