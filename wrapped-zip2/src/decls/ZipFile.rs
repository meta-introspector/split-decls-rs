macro_rules! deps {
    () => {
        ZipFileReader!();
        ZipFileData!();
    };
}

macro_rules! ZipFile {
    () => {
        deps!();
        # [doc = " A struct for reading a zip file"] pub struct ZipFile < 'a , R : Read > { pub (crate) data : Cow < 'a , ZipFileData > , pub (crate) reader : ZipFileReader < 'a , R > , }
    };
}

ZipFile!();