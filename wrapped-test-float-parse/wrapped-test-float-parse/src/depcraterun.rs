// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [doc = " Collect, filter, and launch all tests."] pub fn run (cfg : Config , include : & [String] , exclude : & [String]) -> ExitCode { let threads = std :: thread :: available_parallelism () . map (Into :: into) . unwrap_or (0) * 3 / 2 ; rayon :: ThreadPoolBuilder :: new () . num_threads (threads) . build_global () . unwrap () ; let mut tests = register_tests (& cfg) ; println ! ("registered") ; let initial_tests : Vec < _ > = tests . iter () . map (| t | t . name . clone ()) . collect () ; let unmatched : Vec < _ > = include . iter () . chain (exclude . iter ()) . filter (| filt | ! tests . iter () . any (| t | t . matches (filt))) . collect () ; assert ! (unmatched . is_empty () , "filters were provided that have no matching tests: {unmatched:#?}") ; tests . retain (| test | ! exclude . iter () . any (| exc | test . matches (exc))) ; if cfg . skip_huge { tests . retain (| test | ! test . is_huge_test ()) ; } if ! include . is_empty () { tests . retain (| test | include . iter () . any (| inc | test . matches (inc))) ; } for exc in initial_tests . iter () . filter (| orig_name | ! tests . iter () . any (| t | t . name == * * orig_name)) { println ! ("Skipping test '{exc}'") ; } println ! ("Launching all") ; let elapsed = launch_tests (& mut tests , & cfg) ; ui :: finish_all (& tests , elapsed , & cfg) }
};
}
