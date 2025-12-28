macro_rules! deps {
    () => {
        HuffmanScratch!();
        HuffmanTable!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl HuffmanScratch { pub fn new () -> HuffmanScratch { HuffmanScratch { table : HuffmanTable :: new () , } } }
    };
}

impl_172!()