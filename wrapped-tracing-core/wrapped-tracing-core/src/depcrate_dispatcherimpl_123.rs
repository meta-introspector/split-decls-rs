// Generated macro for impl_123 (impl)
macro_rules! Depcrate_dispatcherimpl_123 {
() => {
// Module: crate::dispatcher
// Provides: {"impl_123"}
// Dependencies: {}
impl Kind < Weak < dyn Subscriber + Send + Sync > > { fn upgrade (& self) -> Option < Kind < Arc < dyn Subscriber + Send + Sync > > > { match self { Kind :: Global (s) => Some (Kind :: Global (* s)) , Kind :: Scoped (ref s) => Some (Kind :: Scoped (s . upgrade () ?)) , } } }
};
}
