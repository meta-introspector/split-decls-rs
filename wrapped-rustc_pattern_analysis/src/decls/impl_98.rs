macro_rules! deps {
    () => {
        Constructor!();
        PlaceValidity!();
        PatCx!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl PlaceValidity { pub fn from_bool (is_valid_only : bool) -> Self { if is_valid_only { ValidOnly } else { MaybeInvalid } } fn is_known_valid (self) -> bool { matches ! (self , ValidOnly) } # [doc = " If the place has validity given by `self` and we read that the value at the place has"] # [doc = " constructor `ctor`, this computes what we can assume about the validity of the constructor"] # [doc = " fields."] # [doc = ""] # [doc = " Pending further opsem decisions, the current behavior is: validity is preserved, except"] # [doc = " inside `&` and union fields where validity is reset to `MaybeInvalid`."] fn specialize < Cx : PatCx > (self , ctor : & Constructor < Cx >) -> Self { if matches ! (ctor , Constructor :: Ref | Constructor :: DerefPattern (_) | Constructor :: UnionField) { MaybeInvalid } else { self } } }
    };
}

impl_98!();