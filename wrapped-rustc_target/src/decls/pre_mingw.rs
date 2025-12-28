macro_rules! deps {
    () => {
        CrtObjects!();
    };
}

macro_rules! pre_mingw {
    () => {
        deps!();
        pub (super) fn pre_mingw () -> CrtObjects { all ("rsbegin.o") }
    };
}

pre_mingw!();