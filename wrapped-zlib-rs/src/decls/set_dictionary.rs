macro_rules! deps {
    () => {
        Mode!();
        InflateStream!();
        Flags!();
        ReturnCode!();
    };
}

macro_rules! set_dictionary {
    () => {
        deps!();
        pub fn set_dictionary (stream : & mut InflateStream , dictionary : & [u8]) -> ReturnCode { if stream . state . wrap != 0 && ! matches ! (stream . state . mode , Mode :: Dict) { return ReturnCode :: StreamError ; } if matches ! (stream . state . mode , Mode :: Dict) { let dictid = adler32 (1 , dictionary) ; if dictid != stream . state . checksum { return ReturnCode :: DataError ; } } let err = 'blk : { if stream . state . window . size () == 0 { match Window :: new_in (& stream . alloc , stream . state . wbits as usize) { None => break 'blk ReturnCode :: MemError , Some (window) => stream . state . window = window , } } stream . state . window . extend (dictionary , stream . state . gzip_flags , false , & mut stream . state . checksum , & mut stream . state . crc_fold ,) ; ReturnCode :: Ok } ; if err != ReturnCode :: Ok { stream . state . mode = Mode :: Mem ; return ReturnCode :: MemError ; } stream . state . flags . update (Flags :: HAVE_DICT , true) ; ReturnCode :: Ok }
    };
}

set_dictionary!()