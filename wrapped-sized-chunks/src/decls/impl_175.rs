macro_rules! deps {
    () => {
        Slice!();
        SliceMut!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq < SliceMut < 'a , A , N > > for Slice < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & SliceMut < 'a , A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_175!();