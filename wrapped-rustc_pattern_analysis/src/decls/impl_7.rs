macro_rules! deps {
    () => {
        MaybeInfiniteInt!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl MaybeInfiniteInt { pub fn new_finite_uint (bits : u128) -> Self { Finite (bits) } pub fn new_finite_int (bits : u128 , size : u64) -> Self { let bias = 1u128 << (size - 1) ; Finite (bits ^ bias) } pub fn as_finite_uint (self) -> Option < u128 > { match self { Finite (bits) => Some (bits) , _ => None , } } pub fn as_finite_int (self , size : u64) -> Option < u128 > { match self { Finite (bits) => { let bias = 1u128 << (size - 1) ; Some (bits ^ bias) } _ => None , } } # [doc = " Note: this will not turn a finite value into an infinite one or vice-versa."] pub fn minus_one (self) -> Option < Self > { match self { Finite (n) => n . checked_sub (1) . map (Finite) , x => Some (x) , } } # [doc = " Note: this will turn `u128::MAX` into `PosInfinity`. This means `plus_one` and `minus_one`"] # [doc = " are not strictly inverses, but that poses no problem in our use of them."] # [doc = " this will not turn a finite value into an infinite one or vice-versa."] pub fn plus_one (self) -> Option < Self > { match self { Finite (n) => match n . checked_add (1) { Some (m) => Some (Finite (m)) , None => Some (PosInfinity) , } , x => Some (x) , } } }
    };
}

impl_7!()