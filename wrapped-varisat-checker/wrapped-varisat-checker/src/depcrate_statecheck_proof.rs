// Generated macro for check_proof (function)
macro_rules! Depcrate_statecheck_proof {
() => {
// Module: crate::state
// Provides: {"check_proof"}
// Dependencies: {}
# [doc = " Checks a proof in the native Varisat format."] pub fn check_proof < 'a > (mut ctx : partial ! (Context <'a >, mut CheckerStateP , mut ClauseHasherP , mut ClausesP , mut ProcessingP <'a >, mut RupCheckP , mut TmpDataP , mut VariablesP ,) , input : impl io :: Read ,) -> Result < () , CheckerError > { let mut buffer = io :: BufReader :: new (input) ; let mut parser = Parser :: default () ; while ! ctx . part (CheckerStateP) . ended { ctx . part_mut (CheckerStateP) . step += 1 ; let step = ctx . part (CheckerStateP) . step ; if step % 100000 == 0 { log :: info ! ("checking step {}k" , step / 1000) ; } match parser . parse_step (& mut buffer) { Ok (step) => check_step (ctx . borrow () , step) ? , Err (err) => match err . downcast :: < io :: Error > () { Ok (io_err) => { if io_err . kind () == io :: ErrorKind :: UnexpectedEof { return Err (CheckerError :: ProofIncomplete { step }) ; } else { return Err (CheckerError :: IoError { step , cause : io_err , }) ; } } Err (err) => return Err (CheckerError :: ParseError { step , cause : err }) , } , } } process_unit_conflicts (ctx . borrow ()) }
};
}
