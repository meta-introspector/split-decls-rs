macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! OptionDrain {
    () => {
        deps!();
        # [doc = " A draining iterator over `Option`s of the elements of a `SparseChunk`."] # [doc = ""] # [doc = " Iterates over every index in the `SparseChunk`, from zero to its full capacity,"] # [doc = " returning an `Option<A>` for each index."] pub struct OptionDrain < A , const N : usize > where BitsImpl < N > : Bits , { pub (crate) index : usize , pub (crate) chunk : SparseChunk < A , N > , }
    };
}

OptionDrain!();