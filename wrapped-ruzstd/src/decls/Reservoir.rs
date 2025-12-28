macro_rules! Reservoir {
    () => {
        # [doc = " A reservoir is created from an input stream."] # [doc = ""] # [doc = " Once filled, it will contain a best effort sample of a dataset,"] # [doc = " where each input value has an equivalent probability of being included."] struct Reservoir { # [doc = " Where the sampled data is stored."] # [doc = ""] # [doc = " Once the lake is filled, then this should contain a representative sample"] # [doc = " of the larger dataset."] lake : Vec < u8 > , # [doc = " K is the size of each sample."] # [doc = ""] # [doc = " The original Zstd dictionary implementation states that values"] # [doc = " between 16 and 2048+ are reasonable."] k : u16 , }
    };
}

Reservoir!();