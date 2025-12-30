// Generated macro for Completed (struct)
macro_rules! DepcrateCompleted {
() => {
// Module: crate
// Provides: {"Completed"}
// Dependencies: {}
# [doc = " Information about a completed test generator."] # [derive (Clone , Debug)] struct Completed { # [doc = " Finished tests (both successful and failed)."] executed : u64 , # [doc = " Failed tests."] failures : u64 , # [doc = " Extra exit information if unsuccessful."] result : Result < FinishedAll , EarlyExit > , # [doc = " If there is something to warn about (e.g bad estimate), leave it here."] warning : Option < Box < str > > , # [doc = " Total time to run the test."] elapsed : Duration , }
};
}
