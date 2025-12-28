macro_rules! deps {
    () => {
        FSETable!();
        Entry!();
    };
}

macro_rules! FSEDecoder {
    () => {
        deps!();
        pub struct FSEDecoder < 'table > { # [doc = " An FSE state value represents an index in the FSE table."] pub state : Entry , # [doc = " A reference to the table used for decoding."] table : & 'table FSETable , }
    };
}

FSEDecoder!()