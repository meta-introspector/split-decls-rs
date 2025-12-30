// Generated macro for Test (trait)
macro_rules! Depcrate_builtins_testersTest {
() => {
// Module: crate::builtins::testers
// Provides: {"Test"}
// Dependencies: {}
# [doc = " The tester function type definition"] pub trait Test : Sync + Send { # [doc = " The tester function type definition"] fn test (& self , value : Option < & Value > , args : & [Value]) -> Result < bool > ; }
};
}
