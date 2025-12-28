macro_rules! deps {
    () => {
        SliceMut!();
        RingBuffer!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < A , const N : usize > PartialEq < SliceMut < '_ , A , N > > for RingBuffer < A , N > where A : PartialEq , { fn eq (& self , other : & SliceMut < '_ , A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_222!()