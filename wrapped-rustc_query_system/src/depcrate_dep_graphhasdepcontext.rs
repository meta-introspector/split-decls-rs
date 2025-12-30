// Generated macro for HasDepContext (trait)
macro_rules! Depcrate_dep_graphHasDepContext {
() => {
// Module: crate::dep_graph
// Provides: {"HasDepContext"}
// Dependencies: {}
pub trait HasDepContext : Copy { type Deps : self :: Deps ; type DepContext : self :: DepContext < Deps = Self :: Deps > ; fn dep_context (& self) -> & Self :: DepContext ; }
};
}
