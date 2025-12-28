macro_rules! find {
    () => {
        pub (crate) fn find () -> Option < Vec < String > > { try_find () . ok () }
    };
}

find!();