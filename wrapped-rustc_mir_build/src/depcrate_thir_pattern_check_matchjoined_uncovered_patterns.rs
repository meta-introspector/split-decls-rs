// Generated macro for joined_uncovered_patterns (function)
macro_rules! Depcrate_thir_pattern_check_matchjoined_uncovered_patterns {
() => {
// Module: crate::thir::pattern::check_match
// Provides: {"joined_uncovered_patterns"}
// Dependencies: {}
fn joined_uncovered_patterns < 'p , 'tcx > (cx : & PatCtxt < 'p , 'tcx > , witnesses : & [WitnessPat < 'p , 'tcx >] ,) -> String { const LIMIT : usize = 3 ; let pat_to_str = | pat : & WitnessPat < 'p , 'tcx > | cx . print_witness_pat (pat) ; match witnesses { [] => bug ! () , [witness] => format ! ("`{}`" , cx . print_witness_pat (witness)) , [head @ .. , tail] if head . len () < LIMIT => { let head : Vec < _ > = head . iter () . map (pat_to_str) . collect () ; format ! ("`{}` and `{}`" , head . join ("`, `") , cx . print_witness_pat (tail)) } _ => { let (head , tail) = witnesses . split_at (LIMIT) ; let head : Vec < _ > = head . iter () . map (pat_to_str) . collect () ; format ! ("`{}` and {} more" , head . join ("`, `") , tail . len ()) } } }
};
}
