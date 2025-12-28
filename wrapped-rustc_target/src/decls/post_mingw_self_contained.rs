macro_rules! deps {
    () => {
        CrtObjects!();
    };
}

macro_rules! post_mingw_self_contained {
    () => {
        deps!();
        pub (super) fn post_mingw_self_contained () -> CrtObjects { all ("rsend.o") }
    };
}

post_mingw_self_contained!();