// Generated macro for RunStrategy (enum)
macro_rules! Depcrate_optionsRunStrategy {
() => {
// Module: crate::options
// Provides: {"RunStrategy"}
// Dependencies: {}
# [derive (Clone , Copy)] pub enum RunStrategy { # [doc = " Runs the test in the current process, and sends the result back over the"] # [doc = " supplied channel."] InProcess , # [doc = " Spawns a subprocess to run the test, and sends the result back over the"] # [doc = " supplied channel. Requires `argv[0]` to exist and point to the binary"] # [doc = " that's currently running."] SpawnPrimary , }
};
}
