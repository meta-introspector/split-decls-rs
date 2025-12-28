macro_rules! deps {
    () => {
        Representation!();
        Answer!();
        Assume!();
    };
}

macro_rules! is_transmutable {
    () => {
        deps!();
        fn is_transmutable < R : Representation + Clone > (src : & R , dst : & R , assume : Assume ,) -> crate :: Answer < ! , ! > { let src = src . clone () ; let dst = dst . clone () ; R :: is_transmutable (src , dst , assume) }
    };
}

is_transmutable!();