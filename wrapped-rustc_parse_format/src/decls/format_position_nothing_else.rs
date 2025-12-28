macro_rules! deps {
    () => {
        Argument!();
    };
}

macro_rules! format_position_nothing_else {
    () => {
        deps!();
        # [test] fn format_position_nothing_else () { same ("{3:}" , & [NextArgument (Box :: new (Argument { position : ArgumentIs (3) , position_span : 2 .. 3 , format : fmtdflt () , }))] ,) ; }
    };
}

format_position_nothing_else!()