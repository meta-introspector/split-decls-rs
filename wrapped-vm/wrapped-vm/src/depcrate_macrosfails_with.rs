// Generated macro for fails_with (macro)
macro_rules! Depcrate_macrosfails_with {
() => {
// Module: crate::macros
// Provides: {"fails_with"}
// Dependencies: {}
# [doc = " A macro that tests input fails to parse."] # [macro_export] macro_rules ! fails_with { (parser : $ parser : expr , input : $ string : expr , rule : $ rule : expr , positives : $ positives : expr , negatives : $ negatives : expr , pos : $ pos : expr) => { # [allow (unused_mut)] # [allow (unused_variables)] { let vm = $ parser ; let error = vm . parse ($ rule , $ string) . unwrap_err () ; match error . variant { :: pest :: error :: ErrorVariant :: ParsingError { positives , negatives , } => { let quoted_positives : Vec <& str > = $ positives ; let quoted_negatives : Vec <& str > = $ negatives ; assert_eq ! (quoted_positives , positives) ; assert_eq ! (quoted_negatives , negatives) ; } _ => unreachable ! () , } ; match error . location { :: pest :: error :: InputLocation :: Pos (pos) => assert_eq ! (pos , $ pos) , _ => unreachable ! () , } } } ; }
};
}
