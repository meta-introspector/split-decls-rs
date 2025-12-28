macro_rules! deps {
    () => {
        InvariantsEq!();
    };
}

macro_rules! unsafe_impl_invariants_eq {
    () => {
        deps!();
        macro_rules ! unsafe_impl_invariants_eq { ($ tyvar : ident => $ t : ty , $ u : ty) => { { crate :: util :: macros :: __unsafe () ; unsafe impl <$ tyvar > InvariantsEq <$ t > for $ u { } unsafe impl <$ tyvar > InvariantsEq <$ u > for $ t { } } } ; }
    };
}

unsafe_impl_invariants_eq!();