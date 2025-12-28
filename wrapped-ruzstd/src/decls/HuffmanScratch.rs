macro_rules! deps {
    () => {
        HuffmanTable!();
    };
}

macro_rules! HuffmanScratch {
    () => {
        deps!();
        pub struct HuffmanScratch { pub table : HuffmanTable , }
    };
}

HuffmanScratch!();