macro_rules! color {
    () => {
        pub (crate) fn color (color : Color) { lock () . set_color (ColorSpec :: new () . set_fg (Some (color))) ; }
    };
}

color!();