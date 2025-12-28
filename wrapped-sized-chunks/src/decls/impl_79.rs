macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > Extend < & 'a A > for Chunk < A , N > where A : 'a + Copy , { # [doc = " Append the contents of the iterator to the back of the chunk."] # [doc = ""] # [doc = " Panics if the chunk exceeds its capacity."] # [doc = ""] # [doc = " Time: O(n) for the length of the iterator"] fn extend < I > (& mut self , it : I) where I : IntoIterator < Item = & 'a A > , { for item in it { self . push_back (* item) ; } } }
    };
}

impl_79!()