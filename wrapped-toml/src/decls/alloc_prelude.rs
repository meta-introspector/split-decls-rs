macro_rules! alloc_prelude {
    () => {
        pub (crate) mod alloc_prelude { pub (crate) use alloc :: borrow :: ToOwned as _ ; pub (crate) use alloc :: format ; pub (crate) use alloc :: string :: String ; pub (crate) use alloc :: string :: ToString as _ ; pub (crate) use alloc :: vec :: Vec ; }
    };
}

alloc_prelude!()