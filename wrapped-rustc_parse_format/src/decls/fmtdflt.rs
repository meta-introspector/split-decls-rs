macro_rules! deps {
    () => {
        FormatSpec!();
    };
}

macro_rules! fmtdflt {
    () => {
        deps!();
        fn fmtdflt () -> FormatSpec < 'static > { return FormatSpec { fill : None , fill_span : None , align : AlignUnknown , sign : None , alternate : false , zero_pad : false , debug_hex : None , precision : CountImplied , width : CountImplied , precision_span : None , width_span : None , ty : "" , ty_span : None , } ; }
    };
}

fmtdflt!();