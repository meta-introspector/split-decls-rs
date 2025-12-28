macro_rules! reset {
    () => {
        pub (crate) fn reset () { lock () . reset () ; }
    };
}

reset!()