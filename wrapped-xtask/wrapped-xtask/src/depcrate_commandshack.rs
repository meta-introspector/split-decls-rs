// Generated macro for hack (function)
macro_rules! Depcrate_commandshack {
() => {
// Module: crate::commands
// Provides: {"hack"}
// Dependencies: {}
# [doc = " Run cargo hack to test each feature in isolation"] fn hack () -> Result < () > { run_cargo (vec ! ["hack" , "test" , "--lib" , "--each-feature" , "--workspace" ,]) }
};
}
