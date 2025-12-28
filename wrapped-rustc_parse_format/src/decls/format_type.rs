macro_rules! deps {
    () => {
        Argument!();
        FormatSpec!();
    };
}

macro_rules! format_type {
    () => {
        deps!();
        # [test] fn format_type () { same ("{3:x}" , & [NextArgument (Box :: new (Argument { position : ArgumentIs (3) , position_span : 2 .. 3 , format : FormatSpec { fill : None , fill_span : None , align : AlignUnknown , sign : None , alternate : false , zero_pad : false , debug_hex : None , precision : CountImplied , width : CountImplied , precision_span : None , width_span : None , ty : "x" , ty_span : None , } , }))] ,) ; }
    };
}

format_type!()