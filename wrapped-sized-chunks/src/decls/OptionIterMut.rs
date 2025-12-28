macro_rules! deps {
    () => {
        SparseChunk!();
    };
}

macro_rules! OptionIterMut {
    () => {
        deps!();
        # [doc = " An iterator over `Option`s of mutable references to the elements of a `SparseChunk`."] # [doc = ""] # [doc = " Iterates over every index in the `SparseChunk`, from zero to its full capacity,"] # [doc = " returning an `Option<&mut A>` for each index."] pub struct OptionIterMut < 'a , A , const N : usize > where BitsImpl < N > : Bits , { pub (crate) index : usize , pub (crate) chunk : & 'a mut SparseChunk < A , N > , }
    };
}

OptionIterMut!();