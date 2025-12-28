macro_rules! deps {
    () => {
        Argument!();
    };
}

macro_rules! format_named_space_nothing {
    () => {
        deps!();
        # [test] fn format_named_space_nothing () { same ("{name} {}" , & [NextArgument (Box :: new (Argument { position : ArgumentNamed ("name") , position_span : 2 .. 6 , format : fmtdflt () , })) , Lit (" ") , NextArgument (Box :: new (Argument { position : ArgumentImplicitlyIs (0) , position_span : 9 .. 9 , format : fmtdflt () , })) ,] ,) }
    };
}

format_named_space_nothing!()