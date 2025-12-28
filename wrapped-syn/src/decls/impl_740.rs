macro_rules! deps {
    () => {
        TokenTreeHelper!();
        TokenStreamHelper!();
    };
}

macro_rules! impl_740 {
    () => {
        deps!();
        impl < 'a > PartialEq for TokenStreamHelper < 'a > { fn eq (& self , other : & Self) -> bool { let left = self . 0 . clone () . into_iter () ; let mut right = other . 0 . clone () . into_iter () ; for item1 in left { let item2 = match right . next () { Some (item) => item , None => return false , } ; if TokenTreeHelper (& item1) != TokenTreeHelper (& item2) { return false ; } } right . next () . is_none () } }
    };
}

impl_740!()