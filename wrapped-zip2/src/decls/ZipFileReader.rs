macro_rules! deps {
    () => {
        Crc32Reader!();
        Decompressor!();
        CryptoReader!();
    };
}

macro_rules! ZipFileReader {
    () => {
        deps!();
        pub (crate) enum ZipFileReader < 'a , R : Read > { NoReader , Raw (io :: Take < & 'a mut R >) , Compressed (Box < Crc32Reader < Decompressor < io :: BufReader < CryptoReader < 'a , R > > > > >) , }
    };
}

ZipFileReader!();