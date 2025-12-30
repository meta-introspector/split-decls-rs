// Generated macro for impl_122 (impl)
macro_rules! Depcrate_dispatcherimpl_122 {
() => {
// Module: crate::dispatcher
// Provides: {"impl_122"}
// Dependencies: {}
impl Kind < Arc < dyn Subscriber + Send + Sync > > { fn downgrade (& self) -> Kind < Weak < dyn Subscriber + Send + Sync > > { match self { Kind :: Global (s) => Kind :: Global (* s) , Kind :: Scoped (ref s) => Kind :: Scoped (Arc :: downgrade (s)) , } } }
};
}
