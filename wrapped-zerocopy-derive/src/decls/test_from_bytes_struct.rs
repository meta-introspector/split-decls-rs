macro_rules! test_from_bytes_struct {
    () => {
        # [test] fn test_from_bytes_struct () { test ! { FromBytes { struct Foo ; } expands to { # [allow (deprecated)] # [automatically_derived] unsafe impl :: zerocopy :: TryFromBytes for Foo { fn only_derive_is_allowed_to_implement_this_trait () { } fn is_bit_valid < ___ZerocopyAliasing > (_candidate : :: zerocopy :: Maybe < Self , ___ZerocopyAliasing >,) -> :: zerocopy :: util :: macro_util :: core_reexport :: primitive :: bool where ___ZerocopyAliasing : :: zerocopy :: pointer :: invariant :: Reference , { if false { fn assert_is_from_bytes < T > () where T : :: zerocopy :: FromBytes , T : ?:: zerocopy :: util :: macro_util :: core_reexport :: marker :: Sized , { } assert_is_from_bytes ::< Self > () ; } true } } # [allow (deprecated)] # [automatically_derived] unsafe impl :: zerocopy :: FromZeros for Foo { fn only_derive_is_allowed_to_implement_this_trait () { } } # [allow (deprecated)] # [automatically_derived] unsafe impl :: zerocopy :: FromBytes for Foo { fn only_derive_is_allowed_to_implement_this_trait () { } } } no_build } }
    };
}

test_from_bytes_struct!();