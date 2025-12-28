macro_rules! ok {
    () => {
        pub (crate) fn ok () { term :: color (Green) ; println ! ("ok") ; term :: reset () ; }
    };
}

ok!()