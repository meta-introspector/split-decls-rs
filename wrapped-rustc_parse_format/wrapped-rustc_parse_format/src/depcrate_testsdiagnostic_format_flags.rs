// Generated macro for diagnostic_format_flags (function)
macro_rules! Depcrate_testsdiagnostic_format_flags {
() => {
// Module: crate::tests
// Provides: {"diagnostic_format_flags"}
// Dependencies: {}
# [test] fn diagnostic_format_flags () { let lit = "{thing:blah}" ; let mut parser = Parser :: new (lit , None , None , false , ParseMode :: Diagnostic) ; assert ! (! parser . is_source_literal) ; let [NextArgument (arg)] = & * parser . by_ref () . collect :: < Vec < Piece < 'static > > > () else { panic ! () } ; assert_eq ! (** arg , Argument { position : ArgumentNamed ("thing") , position_span : 2 .. 7 , format : FormatSpec { ty : ":blah" , ty_span : Some (7 .. 12) , .. Default :: default () } , }) ; assert_eq ! (parser . line_spans , & []) ; assert ! (parser . errors . is_empty ()) ; }
};
}
