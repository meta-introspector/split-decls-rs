macro_rules! deps {
    () => {
        ZipResult!();
        ZipFileData!();
    };
}

macro_rules! find_content {
    () => {
        deps!();
        pub (crate) fn find_content < 'a , R : Read + Seek > (data : & ZipFileData , reader : & 'a mut R ,) -> ZipResult < io :: Take < & 'a mut R > > { let data_start = data . data_start (reader) ? ; reader . seek (SeekFrom :: Start (data_start)) ? ; Ok (reader . take (data . compressed_size)) }
    };
}

find_content!()