macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < A , const N : usize > Extend < A > for Chunk < A , N > { # [doc = " Append the contents of the iterator to the back of the chunk."] # [doc = ""] # [doc = " Panics if the chunk exceeds its capacity."] # [doc = ""] # [doc = " Time: O(n) for the length of the iterator"] fn extend < I > (& mut self , it : I) where I : IntoIterator < Item = A > , { for item in it { self . push_back (item) ; } } }
    };
}

impl_78!()