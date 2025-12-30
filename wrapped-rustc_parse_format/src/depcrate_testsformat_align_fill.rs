// Generated macro for format_align_fill (function)
macro_rules! Depcrate_testsformat_align_fill {
() => {
// Module: crate::tests
// Provides: {"format_align_fill"}
// Dependencies: {}
# [test] fn format_align_fill () { same ("{3:>}" , & [NextArgument (Box :: new (Argument { position : ArgumentIs (3) , position_span : 2 .. 3 , format : FormatSpec { fill : None , fill_span : None , align : AlignRight , sign : None , alternate : false , zero_pad : false , debug_hex : None , precision : CountImplied , width : CountImplied , precision_span : None , width_span : None , ty : "" , ty_span : None , } , }))] ,) ; same ("{3:0<}" , & [NextArgument (Box :: new (Argument { position : ArgumentIs (3) , position_span : 2 .. 3 , format : FormatSpec { fill : Some ('0') , fill_span : Some (4 .. 5) , align : AlignLeft , sign : None , alternate : false , zero_pad : false , debug_hex : None , precision : CountImplied , width : CountImplied , precision_span : None , width_span : None , ty : "" , ty_span : None , } , }))] ,) ; same ("{3:*<abcd}" , & [NextArgument (Box :: new (Argument { position : ArgumentIs (3) , position_span : 2 .. 3 , format : FormatSpec { fill : Some ('*') , fill_span : Some (4 .. 5) , align : AlignLeft , sign : None , alternate : false , zero_pad : false , debug_hex : None , precision : CountImplied , width : CountImplied , precision_span : None , width_span : None , ty : "abcd" , ty_span : Some (6 .. 10) , } , }))] ,) ; }
};
}
