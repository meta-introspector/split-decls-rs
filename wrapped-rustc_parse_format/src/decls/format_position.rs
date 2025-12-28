macro_rules! deps {
    () => {
        Argument!();
    };
}

macro_rules! format_position {
    () => {
        deps!();
        # [test] fn format_position () { same ("{3}" , & [NextArgument (Box :: new (Argument { position : ArgumentIs (3) , position_span : 2 .. 3 , format : fmtdflt () , }))] ,) ; }
    };
}

format_position!();