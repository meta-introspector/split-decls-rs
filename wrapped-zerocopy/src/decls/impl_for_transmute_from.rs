macro_rules! deps {
    () => {
        FromZeros!();
        TryFromBytes!();
        Maybe!();
        IntoBytes!();
        Exclusive!();
        BecauseExclusive!();
        TransmuteFrom!();
        Valid!();
        FromBytes!();
        Reference!();
    };
}

macro_rules! impl_for_transmute_from {
    () => {
        deps!();
        # [doc = " Implements `$trait` for `$ty` where `$ty: TransmuteFrom<$repr>` (and"] # [doc = " vice-versa)."] # [doc = ""] # [doc = " Calling this macro is safe; the internals of the macro emit appropriate"] # [doc = " trait bounds which ensure that the given impl is sound."] macro_rules ! impl_for_transmute_from { ($ (# [$ attr : meta]) * $ ($ tyvar : ident $ (: $ (? $ optbound : ident $ (+) ?) * $ ($ bound : ident $ (+) ?) *) ?) ? => $ trait : ident for $ ty : ty [$ ($ unsafe_cell : ident) ? <$ repr : ty >]) => { const _ : () = { $ (# [$ attr]) * # [allow (non_local_definitions)] unsafe impl <$ ($ tyvar $ (: $ (? $ optbound +) * $ ($ bound +) *) ?) ?> $ trait for $ ty { # [allow (dead_code , clippy :: missing_inline_in_public_items)] # [cfg_attr (all (coverage_nightly , __ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS) , coverage (off))] fn only_derive_is_allowed_to_implement_this_trait () { use crate :: pointer :: { *, invariant :: Valid } ; impl_for_transmute_from ! (@ assert_is_supported_trait $ trait) ; fn is_trait < T , R > () where T : TransmuteFrom < R , Valid , Valid > + ? Sized , R : TransmuteFrom < T , Valid , Valid > + ? Sized , R : $ trait , { } # [cfg_attr (all (coverage_nightly , __ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS) , coverage (off))] fn f <$ ($ tyvar $ (: $ (? $ optbound +) * $ ($ bound +) *) ?) ?> () { is_trait ::<$ ty , $ repr > () ; } } impl_for_transmute_from ! (@ is_bit_valid $ (<$ tyvar $ (: $ (? $ optbound +) * $ ($ bound +) *) ?>) ? $ trait for $ ty [$ ($ unsafe_cell) ? <$ repr >]) ; } } ; } ; (@ assert_is_supported_trait TryFromBytes) => { } ; (@ assert_is_supported_trait FromZeros) => { } ; (@ assert_is_supported_trait FromBytes) => { } ; (@ assert_is_supported_trait IntoBytes) => { } ; (@ is_bit_valid $ (<$ tyvar : ident $ (: $ (? $ optbound : ident $ (+) ?) * $ ($ bound : ident $ (+) ?) *) ?>) ? TryFromBytes for $ ty : ty [UnsafeCell <$ repr : ty >]) => { # [inline] fn is_bit_valid < A : crate :: pointer :: invariant :: Reference > (candidate : Maybe <'_ , Self , A >) -> bool { let c : Maybe <'_ , Self , crate :: pointer :: invariant :: Exclusive > = candidate . into_exclusive_or_pme () ; let c : Maybe <'_ , $ repr , _ > = c . transmute ::< _ , _ , (_ , (_ , (BecauseExclusive , BecauseExclusive))) > () ; <$ repr as TryFromBytes >:: is_bit_valid (c) } } ; (@ is_bit_valid $ (<$ tyvar : ident $ (: $ (? $ optbound : ident $ (+) ?) * $ ($ bound : ident $ (+) ?) *) ?>) ? TryFromBytes for $ ty : ty [<$ repr : ty >]) => { # [inline] fn is_bit_valid < A : crate :: pointer :: invariant :: Reference > (candidate : $ crate :: Maybe <'_ , Self , A >) -> bool { <$ repr as TryFromBytes >:: is_bit_valid (candidate . transmute ()) } } ; (@ is_bit_valid $ (<$ tyvar : ident $ (: $ (? $ optbound : ident $ (+) ?) * $ ($ bound : ident $ (+) ?) *) ?>) ? $ trait : ident for $ ty : ty [$ ($ unsafe_cell : ident) ? <$ repr : ty >]) => { } ; }
    };
}

impl_for_transmute_from!();