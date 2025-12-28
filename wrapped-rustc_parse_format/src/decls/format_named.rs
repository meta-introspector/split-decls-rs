macro_rules! deps {
    () => {
        Argument!();
    };
}

macro_rules! format_named {
    () => {
        deps!();
        # [test] fn format_named () { same ("{name}" , & [NextArgument (Box :: new (Argument { position : ArgumentNamed ("name") , position_span : 2 .. 6 , format : fmtdflt () , }))] ,) }
    };
}

format_named!();