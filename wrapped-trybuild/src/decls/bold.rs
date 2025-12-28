macro_rules! bold {
    () => {
        pub (crate) fn bold () { lock () . set_color (ColorSpec :: new () . set_bold (true)) ; }
    };
}

bold!()