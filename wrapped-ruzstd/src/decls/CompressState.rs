macro_rules! deps {
    () => {
        FseTables!();
        Matcher!();
        HuffmanTable!();
    };
}

macro_rules! CompressState {
    () => {
        deps!();
        pub (crate) struct CompressState < M : Matcher > { pub (crate) matcher : M , pub (crate) last_huff_table : Option < crate :: huff0 :: huff0_encoder :: HuffmanTable > , pub (crate) fse_tables : FseTables , }
    };
}

CompressState!();