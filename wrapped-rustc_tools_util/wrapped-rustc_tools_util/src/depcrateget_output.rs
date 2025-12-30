// Generated macro for get_output (function)
macro_rules! Depcrateget_output {
() => {
// Module: crate
// Provides: {"get_output"}
// Dependencies: {}
# [must_use] fn get_output (cmd : & str , args : & [& str]) -> Option < String > { let output = Command :: new (cmd) . args (args) . output () . ok () ? ; let mut stdout = output . status . success () . then_some (output . stdout) ? ; while stdout . last () . copied () == Some (b'\n') { stdout . pop () ; } String :: from_utf8 (stdout) . ok () }
};
}
