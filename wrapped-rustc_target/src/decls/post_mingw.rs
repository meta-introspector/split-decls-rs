macro_rules! deps {
    () => {
        CrtObjects!();
    };
}

macro_rules! post_mingw {
    () => {
        deps!();
        pub (super) fn post_mingw () -> CrtObjects { all ("rsend.o") }
    };
}

post_mingw!();