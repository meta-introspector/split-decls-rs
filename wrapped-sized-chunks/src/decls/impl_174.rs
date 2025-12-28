macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq for Slice < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_174!();