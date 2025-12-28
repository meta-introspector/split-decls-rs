macro_rules! deps {
    () => {
        SourceFile!();
    };
}

macro_rules! CacheEntry {
    () => {
        deps!();
        # [derive (Clone)] struct CacheEntry { time_stamp : usize , line_number : usize , line : Range < BytePos > , file : Arc < SourceFile > , file_index : usize , }
    };
}

CacheEntry!()