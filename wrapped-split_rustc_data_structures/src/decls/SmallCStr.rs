macro_rules! SmallCStr {
    () => {
        # [doc = " Like SmallVec but for C strings."] # [derive (Clone)] pub struct SmallCStr { data : SmallVec < [u8 ; SIZE] > , }
    };
}

SmallCStr!()