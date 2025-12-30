// Generated macro for Test (trait)
macro_rules! DepcrateTest {
() => {
// Module: crate
// Provides: {"Test"}
// Dependencies: {}
# [doc = " Function signature for running a test [`Case`]"] pub trait Test < S , E > where S : std :: fmt :: Display , E : std :: fmt :: Display , { fn run (& self , fixture : & std :: path :: Path) -> Result < S , E > ; }
};
}
