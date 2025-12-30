// Generated macro for impl_109 (impl)
macro_rules! Depcrate_cycleimpl_109 {
() => {
// Module: crate::cycle
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'db > ProvisionalStatus < 'db > { pub (crate) fn cycle_heads (& self) -> & 'db CycleHeads { match self { ProvisionalStatus :: Provisional { cycle_heads , .. } => cycle_heads , _ => empty_cycle_heads () , } } pub (crate) const fn is_provisional (& self) -> bool { matches ! (self , ProvisionalStatus :: Provisional { .. }) } }
};
}
