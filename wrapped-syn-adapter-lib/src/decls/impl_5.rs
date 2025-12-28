macro_rules! deps {
    () => {
        LibSynAdapter!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        # [cfg (feature = "syn-parsing")] impl LibSynAdapter { pub fn new () -> Self { LibSynAdapter } }
    };
}

impl_5!()