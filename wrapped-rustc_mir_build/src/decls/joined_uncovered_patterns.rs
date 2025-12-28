macro_rules! deps {
    () => {
        PatCtxt!();
    };
}

macro_rules! joined_uncovered_patterns {
    () => {
        deps!();
        fn joined_uncovered_patterns < 'p , 'tcx > (cx : & PatCtxt < 'p , 'tcx > , witnesses : & [WitnessPat < 'p , 'tcx >] ,) -> String { const LIMIT : usize = 3 ; let pat_to_str = | pat : & WitnessPat < 'p , 'tcx > | cx . print_witness_pat (pat) ; match witnesses { [] => bug ! () , [witness] => format ! ("`{}`" , cx . print_witness_pat (witness)) , [head @ .. , tail] if head . len () < LIMIT => { let head : Vec < _ > = head . iter () . map (pat_to_str) . collect () ; format ! ("`{}` and `{}`" , head . join ("`, `") , cx . print_witness_pat (tail)) } _ => { let (head , tail) = witnesses . split_at (LIMIT) ; let head : Vec < _ > = head . iter () . map (pat_to_str) . collect () ; format ! ("`{}` and {} more" , head . join ("`, `") , tail . len ()) } } }
    };
}

joined_uncovered_patterns!()