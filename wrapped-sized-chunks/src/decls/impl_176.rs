macro_rules! deps {
    () => {
        Slice!();
        RingBuffer!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq < RingBuffer < A , N > > for Slice < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & RingBuffer < A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_176!()