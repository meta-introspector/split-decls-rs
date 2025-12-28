macro_rules! test_try_from_bytes {
    () => {
        # [test] fn test_try_from_bytes () { test ! { TryFromBytes { struct Foo ; } expands to { # [allow (deprecated)] # [automatically_derived] unsafe impl :: zerocopy :: TryFromBytes for Foo { fn only_derive_is_allowed_to_implement_this_trait () { } fn is_bit_valid < ___ZerocopyAliasing > (mut candidate : :: zerocopy :: Maybe < Self , ___ZerocopyAliasing >,) -> :: zerocopy :: util :: macro_util :: core_reexport :: primitive :: bool where ___ZerocopyAliasing : :: zerocopy :: pointer :: invariant :: Reference , { use :: zerocopy :: util :: macro_util :: core_reexport ; use :: zerocopy :: pointer :: PtrInner ; true } } } no_build } }
    };
}

test_try_from_bytes!()