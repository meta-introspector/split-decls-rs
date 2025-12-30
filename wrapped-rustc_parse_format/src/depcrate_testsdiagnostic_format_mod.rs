// Generated macro for diagnostic_format_mod (function)
macro_rules! Depcrate_testsdiagnostic_format_mod {
() => {
// Module: crate::tests
// Provides: {"diagnostic_format_mod"}
// Dependencies: {}
# [test] fn diagnostic_format_mod () { let lit = "{thing:+}" ; let mut parser = Parser :: new (lit , None , None , false , ParseMode :: Diagnostic) ; assert ! (! parser . is_source_literal) ; let [NextArgument (arg)] = & * parser . by_ref () . collect :: < Vec < Piece < 'static > > > () else { panic ! () } ; assert_eq ! (** arg , Argument { position : ArgumentNamed ("thing") , position_span : 2 .. 7 , format : FormatSpec { ty : ":+" , ty_span : Some (7 .. 9) , .. Default :: default () } , }) ; assert_eq ! (parser . line_spans , & []) ; assert ! (parser . errors . is_empty ()) ; }
};
}
