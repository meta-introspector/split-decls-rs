// Generated macro for format_mixture (function)
macro_rules! Depcrate_testsformat_mixture {
() => {
// Module: crate::tests
// Provides: {"format_mixture"}
// Dependencies: {}
# [test] fn format_mixture () { same ("abcd {3:x} efg" , & [Lit ("abcd ") , NextArgument (Box :: new (Argument { position : ArgumentIs (3) , position_span : 7 .. 8 , format : FormatSpec { fill : None , fill_span : None , align : AlignUnknown , sign : None , alternate : false , zero_pad : false , debug_hex : None , precision : CountImplied , width : CountImplied , precision_span : None , width_span : None , ty : "x" , ty_span : None , } , })) , Lit (" efg") ,] ,) ; }
};
}
