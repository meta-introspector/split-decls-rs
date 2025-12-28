macro_rules! deps {
    () => {
        State!();
        HuffmanTable!();
    };
}

macro_rules! HuffmanDecoder {
    () => {
        deps!();
        pub struct HuffmanDecoder < 'table > { table : & 'table HuffmanTable , # [doc = " State is used to index into the table."] pub state : u64 , }
    };
}

HuffmanDecoder!();