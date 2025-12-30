// Generated macro for parses_to (macro)
macro_rules! Depcrate_macrosparses_to {
() => {
// Module: crate::macros
// Provides: {"parses_to"}
// Dependencies: {}
# [doc = " A macro that tests input parses to the expected tokens."] # [macro_export] macro_rules ! parses_to { (parser : $ parser : expr , input : $ string : expr , rule : $ rule : expr , tokens : [$ ($ names : ident $ calls : tt) ,* $ (,) *]) => { # [allow (unused_mut)] { let vm = $ parser ; let mut tokens = vm . parse ($ rule , $ string) . unwrap () . tokens () ; consumes_to ! (& mut tokens , [$ ($ names $ calls) ,*]) ; let rest : Vec < _ > = tokens . collect () ; match rest . len () { 0 => () , 2 => { let (first , second) = (& rest [0] , & rest [1]) ; match (first , second) { (&:: pest :: Token :: Start { rule : ref first_rule , .. } , &:: pest :: Token :: End { rule : ref second_rule , .. }) => { assert ! (format ! ("{}" , first_rule) == "EOI" , "expected end of input, but found {:?}" , rest) ; assert ! (format ! ("{}" , second_rule) == "EOI" , "expected end of input, but found {:?}" , rest) ; } _ => panic ! ("expected end of input, but found {:?}" , rest) } } _ => panic ! ("expected end of input, but found {:?}" , rest) } ; } } ; }
};
}
