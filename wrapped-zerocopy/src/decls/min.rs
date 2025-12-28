macro_rules! min {
    () => {
        pub (crate) const fn min (a : NonZeroUsize , b : NonZeroUsize) -> NonZeroUsize { if a . get () > b . get () { b } else { a } }
    };
}

min!()