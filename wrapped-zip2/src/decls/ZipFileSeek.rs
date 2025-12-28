macro_rules! deps {
    () => {
        ZipFileData!();
        ZipFileSeekReader!();
    };
}

macro_rules! ZipFileSeek {
    () => {
        deps!();
        # [doc = " A struct for reading and seeking a zip file"] pub struct ZipFileSeek < 'a , R > { data : Cow < 'a , ZipFileData > , reader : ZipFileSeekReader < 'a , R > , }
    };
}

ZipFileSeek!()