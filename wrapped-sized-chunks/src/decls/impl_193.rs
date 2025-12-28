macro_rules! deps {
    () => {
        SliceMut!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq for SliceMut < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_193!()