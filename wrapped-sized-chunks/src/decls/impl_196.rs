macro_rules! deps {
    () => {
        SliceMut!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl < 'a , A : PartialEq + 'a , S , const N : usize > PartialEq < S > for SliceMut < 'a , A , N > where S : Borrow < [A] > , { # [inline] # [must_use] fn eq (& self , other : & S) -> bool { let other = other . borrow () ; self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_196!();