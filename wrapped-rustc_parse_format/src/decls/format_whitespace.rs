macro_rules! deps {
    () => {
        Argument!();
    };
}

macro_rules! format_whitespace {
    () => {
        deps!();
        # [test] fn format_whitespace () { same ("{ }" , & [NextArgument (Box :: new (Argument { position : ArgumentImplicitlyIs (0) , position_span : 2 .. 3 , format : fmtdflt () , }))] ,) ; same ("{  }" , & [NextArgument (Box :: new (Argument { position : ArgumentImplicitlyIs (0) , position_span : 2 .. 4 , format : fmtdflt () , }))] ,) ; }
    };
}

format_whitespace!()