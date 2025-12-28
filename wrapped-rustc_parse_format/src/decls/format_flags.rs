macro_rules! deps {
    () => {
        Sign!();
        Argument!();
        FormatSpec!();
    };
}

macro_rules! format_flags {
    () => {
        deps!();
        # [test] fn format_flags () { same ("{:-}" , & [NextArgument (Box :: new (Argument { position : ArgumentImplicitlyIs (0) , position_span : 2 .. 2 , format : FormatSpec { fill : None , fill_span : None , align : AlignUnknown , sign : Some (Sign :: Minus) , alternate : false , zero_pad : false , debug_hex : None , precision : CountImplied , width : CountImplied , precision_span : None , width_span : None , ty : "" , ty_span : None , } , }))] ,) ; same ("{:+#}" , & [NextArgument (Box :: new (Argument { position : ArgumentImplicitlyIs (0) , position_span : 2 .. 2 , format : FormatSpec { fill : None , fill_span : None , align : AlignUnknown , sign : Some (Sign :: Plus) , alternate : true , zero_pad : false , debug_hex : None , precision : CountImplied , width : CountImplied , precision_span : None , width_span : None , ty : "" , ty_span : None , } , }))] ,) ; }
    };
}

format_flags!()