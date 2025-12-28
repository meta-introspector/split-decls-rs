macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < S1 : AsRef < str > , S2 : AsRef < str > > PartialEq < Unicode < S2 > > for Unicode < S1 > { # [inline] fn eq (& self , other : & Unicode < S2 >) -> bool { let mut left = self . 0 . as_ref () . chars () . flat_map (lookup) ; let mut right = other . 0 . as_ref () . chars () . flat_map (lookup) ; loop { let x = match left . next () { None => return right . next () . is_none () , Some (val) => val , } ; let y = match right . next () { None => return false , Some (val) => val , } ; if x != y { return false ; } } } }
    };
}

impl_23!();