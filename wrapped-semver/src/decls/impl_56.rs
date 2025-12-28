macro_rules! deps {
    () => {
        Prerelease!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Ord for Prerelease { fn cmp (& self , rhs : & Self) -> Ordering { if self . identifier . ptr_eq (& rhs . identifier) { return Ordering :: Equal ; } match self . is_empty () { true => return Ordering :: Greater , false if rhs . is_empty () => return Ordering :: Less , false => { } } let lhs = self . as_str () . split ('.') ; let mut rhs = rhs . as_str () . split ('.') ; for lhs in lhs { let rhs = match rhs . next () { None => return Ordering :: Greater , Some (rhs) => rhs , } ; let string_cmp = | | Ord :: cmp (lhs , rhs) ; let is_ascii_digit = | b : u8 | b . is_ascii_digit () ; let ordering = match (lhs . bytes () . all (is_ascii_digit) , rhs . bytes () . all (is_ascii_digit) ,) { (true , true) => Ord :: cmp (& lhs . len () , & rhs . len ()) . then_with (string_cmp) , (true , false) => return Ordering :: Less , (false , true) => return Ordering :: Greater , (false , false) => string_cmp () , } ; if ordering != Ordering :: Equal { return ordering ; } } if rhs . next () . is_none () { Ordering :: Equal } else { Ordering :: Less } } }
    };
}

impl_56!();