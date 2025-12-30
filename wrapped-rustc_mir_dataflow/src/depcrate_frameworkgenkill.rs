// Generated macro for GenKill (trait)
macro_rules! Depcrate_frameworkGenKill {
() => {
// Module: crate::framework
// Provides: {"GenKill"}
// Dependencies: {}
# [doc = " The legal operations for a transfer function in a gen/kill problem."] pub trait GenKill < T > { # [doc = " Inserts `elem` into the state vector."] fn gen_ (& mut self , elem : T) ; # [doc = " Removes `elem` from the state vector."] fn kill (& mut self , elem : T) ; # [doc = " Calls `gen` for each element in `elems`."] fn gen_all (& mut self , elems : impl IntoIterator < Item = T >) { for elem in elems { self . gen_ (elem) ; } } # [doc = " Calls `kill` for each element in `elems`."] fn kill_all (& mut self , elems : impl IntoIterator < Item = T >) { for elem in elems { self . kill (elem) ; } } }
};
}
