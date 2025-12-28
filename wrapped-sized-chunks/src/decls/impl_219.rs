macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl < A : PartialEq , const N : usize > PartialEq for RingBuffer < A , N > { # [inline] # [must_use] fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_219!()