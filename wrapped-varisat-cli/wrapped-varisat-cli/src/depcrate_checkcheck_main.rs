// Generated macro for check_main (function)
macro_rules! Depcrate_checkcheck_main {
() => {
// Module: crate::check
// Provides: {"check_main"}
// Dependencies: {}
pub fn check_main (matches : & ArgMatches) -> Result < i32 , Error > { init_logging () ; banner () ; let mut checker = Checker :: default () ; let stdin = io :: stdin () ; let mut locked_stdin ; let mut opened_file ; let file = match matches . value_of ("INPUT") { Some (path) => { log :: info ! ("Reading file '{}'" , path) ; opened_file = fs :: File :: open (path) ? ; & mut opened_file as & mut dyn io :: Read } None => { log :: info ! ("Reading from stdin") ; locked_stdin = stdin . lock () ; & mut locked_stdin as & mut dyn io :: Read } } ; let mut transcript = transcript :: Transcript :: default () ; checker . add_transcript (& mut transcript) ; let mut lrat_processor ; if let Some (lrat_path) = matches . value_of ("lrat-file") { lrat_processor = WriteLrat :: new (fs :: File :: create (lrat_path) ? , false) ; checker . add_processor (& mut lrat_processor) ; } let mut clrat_processor ; if let Some (clrat_path) = matches . value_of ("clrat-file") { clrat_processor = WriteLrat :: new (fs :: File :: create (clrat_path) ? , true) ; checker . add_processor (& mut clrat_processor) ; } checker . add_dimacs_cnf (file) ? ; let path = matches . value_of ("proof-file") . unwrap () ; log :: info ! ("Checking proof file '{}'" , path) ; match checker . check_proof (fs :: File :: open (path) ?) { Ok (()) => println ! ("s VERIFIED") , Err (err) => { log :: error ! ("{}" , err) ; if let CheckerError :: CheckFailed { debug_step , .. } = err { if ! debug_step . is_empty () { log :: error ! ("failed step was {}" , debug_step) } } println ! ("s NOT VERIFIED") ; return Ok (1) ; } } Ok (0) }
};
}
