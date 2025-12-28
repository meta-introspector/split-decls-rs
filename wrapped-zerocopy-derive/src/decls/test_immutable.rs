macro_rules! test_immutable {
    () => {
        # [test] fn test_immutable () { test ! { Immutable { struct Foo ; } expands to { # [allow (deprecated)] # [automatically_derived] unsafe impl :: zerocopy :: Immutable for Foo { fn only_derive_is_allowed_to_implement_this_trait () { } } } no_build } }
    };
}

test_immutable!()