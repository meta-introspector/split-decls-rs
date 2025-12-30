// Generated macro for Reactor (trait)
macro_rules! Depcrate_reactor_traitsReactor {
() => {
// Module: crate::reactor::traits
// Provides: {"Reactor"}
// Dependencies: {}
# [doc = " A reactor worker."] pub trait Reactor : Future < Output = () > { # [doc = " The Reactor Scope"] type Scope : ReactorScoped ; # [doc = " Creates a reactor worker."] fn create (scope : Self :: Scope) -> Self ; }
};
}
