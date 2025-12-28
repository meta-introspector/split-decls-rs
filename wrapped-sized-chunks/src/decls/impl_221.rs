macro_rules! deps {
    () => {
        RingBuffer!();
        Slice!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < A , const N : usize > PartialEq < Slice < '_ , A , N > > for RingBuffer < A , N > where A : PartialEq , { fn eq (& self , other : & Slice < '_ , A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_221!()