macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < A , T > Extend < A > for InlineArray < A , T > { # [doc = " Append the contents of the iterator to the back of the array."] # [doc = ""] # [doc = " Panics if the array exceeds its capacity."] # [doc = ""] # [doc = " Time: O(n) for the length of the iterator"] fn extend < I > (& mut self , it : I) where I : IntoIterator < Item = A > , { for item in it { self . push (item) ; } } }
    };
}

impl_33!()