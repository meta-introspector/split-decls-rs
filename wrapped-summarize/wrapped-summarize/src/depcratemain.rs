// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Box < dyn Error + Send + Sync > > { let opt = Opt :: parse () ; match opt { Opt :: Summarize (opt) => summarize (opt) , Opt :: Diff (opt) => diff (opt) , Opt :: Aggregate (opt) => aggregate (opt) , } }
};
}
