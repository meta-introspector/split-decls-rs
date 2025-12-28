macro_rules! deps {
    () => {
        HuffmanScratch!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl Default for HuffmanScratch { fn default () -> Self { Self :: new () } }
    };
}

impl_173!();