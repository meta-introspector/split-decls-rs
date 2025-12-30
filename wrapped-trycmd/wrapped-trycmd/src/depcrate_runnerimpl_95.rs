// Generated macro for impl_95 (impl)
macro_rules! Depcrate_runnerimpl_95 {
() => {
// Module: crate::runner
// Provides: {"impl_95"}
// Dependencies: {}
impl std :: fmt :: Display for FileStatus { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let palette = snapbox :: report :: Palette :: color () ; match & self { Self :: Ok { expected_path , actual_path : _actual_path , } => { writeln ! (f , "{}: is {}" , expected_path . display () , palette . info ("good") ,) ? ; } Self :: Failure (msg) => { writeln ! (f , "{}" , palette . error (msg)) ? ; } Self :: TypeMismatch { expected_path , actual_path : _actual_path , expected_type , actual_type , } => { writeln ! (f , "{}: Expected {}, was {}" , expected_path . display () , palette . info (expected_type) , palette . error (actual_type)) ? ; } Self :: LinkMismatch { expected_path , actual_path : _actual_path , expected_target , actual_target , } => { writeln ! (f , "{}: Expected {}, was {}" , expected_path . display () , palette . info (expected_target . display ()) , palette . error (actual_target . display ())) ? ; } Self :: ContentMismatch { expected_path , actual_path , expected_content , actual_content , } => { snapbox :: report :: write_diff (f , expected_content , actual_content , Some (& expected_path . display ()) , Some (& actual_path . display ()) , palette ,) ? ; } } Ok (()) } }
};
}
