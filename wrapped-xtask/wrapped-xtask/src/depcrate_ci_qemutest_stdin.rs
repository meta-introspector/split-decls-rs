// Generated macro for test_stdin (function)
macro_rules! Depcrate_ci_qemutest_stdin {
() => {
// Module: crate::ci::qemu
// Provides: {"test_stdin"}
// Dependencies: {}
fn test_stdin (child : & mut Child) -> Result < () > { thread :: sleep (Duration :: from_secs (10)) ; let messages = ["Hello, there!" , "Hello, again!" , "Bye-bye!"] ; let mut stdin = child . stdin . take () . unwrap () ; for message in messages { writeln ! (& mut stdin , "{message}") ? ; stdin . flush () ? ; thread :: sleep (Duration :: from_secs (1)) ; } child . kill () ? ; let stdout = child . stdout . take () . unwrap () ; let stdout_lines = BufReader :: new (stdout) . lines () . collect :: < Result < Vec < _ > , _ > > () ? ; for line in & stdout_lines { println ! ("{line}") ; } for message in messages { assert ! (stdout_lines . iter () . any (| line | line . contains (message))) ; } Ok (()) }
};
}
