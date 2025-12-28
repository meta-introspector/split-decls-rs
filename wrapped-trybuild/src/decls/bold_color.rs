macro_rules! bold_color {
    () => {
        pub (crate) fn bold_color (color : Color) { lock () . set_color (ColorSpec :: new () . set_bold (true) . set_fg (Some (color))) ; }
    };
}

bold_color!()