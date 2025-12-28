macro_rules! deps {
    () => {
        SeekableTake!();
        ZipFileData!();
        ZipResult!();
    };
}

macro_rules! find_content_seek {
    () => {
        deps!();
        fn find_content_seek < 'a , R : Read + Seek > (data : & ZipFileData , reader : & 'a mut R ,) -> ZipResult < SeekableTake < 'a , R > > { let data_start = data . data_start (reader) ? ; reader . seek (SeekFrom :: Start (data_start)) ? ; Ok (SeekableTake :: new (reader , data . compressed_size) ?) }
    };
}

find_content_seek!();