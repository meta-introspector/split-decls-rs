macro_rules! deps {
    () => {
        FSETable!();
    };
}

macro_rules! FSEScratch {
    () => {
        deps!();
        pub struct FSEScratch { pub offsets : FSETable , pub of_rle : Option < u8 > , pub literal_lengths : FSETable , pub ll_rle : Option < u8 > , pub match_lengths : FSETable , pub ml_rle : Option < u8 > , }
    };
}

FSEScratch!()