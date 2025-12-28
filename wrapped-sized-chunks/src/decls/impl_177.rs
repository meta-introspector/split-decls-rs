macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < 'a , A : PartialEq + 'a , S , const N : usize > PartialEq < S > for Slice < 'a , A , N > where S : Borrow < [A] > , { # [inline] # [must_use] fn eq (& self , other : & S) -> bool { let other = other . borrow () ; self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_177!();