macro_rules! max {
    () => {
        pub (crate) const fn max (a : NonZeroUsize , b : NonZeroUsize) -> NonZeroUsize { if a . get () < b . get () { b } else { a } }
    };
}

max!()