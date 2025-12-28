macro_rules! deps {
    () => {
        Filters!();
    };
}

macro_rules! MtStreamBuilder {
    () => {
        deps!();
        # [doc = " Builder to create a multi-threaded stream encoder."] pub struct MtStreamBuilder { raw : lzma_sys :: lzma_mt , filters : Option < Filters > , }
    };
}

MtStreamBuilder!()