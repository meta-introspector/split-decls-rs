macro_rules! test_unaligned {
    () => {
        # [test] fn test_unaligned () { test ! { Unaligned { # [repr (C)] struct Foo ; } expands to { # [allow (deprecated)] # [automatically_derived] unsafe impl :: zerocopy :: Unaligned for Foo { fn only_derive_is_allowed_to_implement_this_trait () { } } } no_build } }
    };
}

test_unaligned!()