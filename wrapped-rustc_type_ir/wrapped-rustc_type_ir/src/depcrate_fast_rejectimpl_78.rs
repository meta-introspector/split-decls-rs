// Generated macro for impl_78 (impl)
macro_rules! Depcrate_fast_rejectimpl_78 {
() => {
// Module: crate::fast_reject
// Provides: {"impl_78"}
// Dependencies: {}
impl < DefId > SimplifiedType < DefId > { pub fn def (self) -> Option < DefId > { match self { SimplifiedType :: Adt (d) | SimplifiedType :: Foreign (d) | SimplifiedType :: Trait (d) | SimplifiedType :: Closure (d) | SimplifiedType :: Coroutine (d) | SimplifiedType :: CoroutineWitness (d) => Some (d) , _ => None , } } }
};
}
