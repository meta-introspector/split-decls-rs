macro_rules! deps {
    () => {
        RingBuffer!();
        SliceMut!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq < RingBuffer < A , N > > for SliceMut < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & RingBuffer < A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_195!();