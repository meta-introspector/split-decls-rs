macro_rules! deps {
    () => {
        Argument!();
    };
}

macro_rules! format_empty {
    () => {
        deps!();
        # [test] fn format_empty () { same ("{}" , & [NextArgument (Box :: new (Argument { position : ArgumentImplicitlyIs (0) , position_span : 2 .. 2 , format : fmtdflt () , }))] ,) ; }
    };
}

format_empty!();