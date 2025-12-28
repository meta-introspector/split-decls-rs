macro_rules! test_eq {
    () => {
        # [test] fn test_eq () { test ! { ByteEq { struct Foo < T : Clone > (T) where Self : Sized ; } expands to { # [allow (deprecated)] # [automatically_derived] impl < T : Clone > :: zerocopy :: util :: macro_util :: core_reexport :: cmp :: PartialEq for Foo < T > where Self : :: zerocopy :: IntoBytes + :: zerocopy :: Immutable , Self : Sized , { fn eq (& self , other : & Self) -> bool { :: zerocopy :: util :: macro_util :: core_reexport :: cmp :: PartialEq :: eq (:: zerocopy :: IntoBytes :: as_bytes (self) , :: zerocopy :: IntoBytes :: as_bytes (other) ,) } } # [allow (deprecated)] # [automatically_derived] impl < T : Clone > :: zerocopy :: util :: macro_util :: core_reexport :: cmp :: Eq for Foo < T > where Self : :: zerocopy :: IntoBytes + :: zerocopy :: Immutable , Self : Sized , { } } no_build } }
    };
}

test_eq!()