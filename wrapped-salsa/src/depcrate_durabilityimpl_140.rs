// Generated macro for impl_140 (impl)
macro_rules! Depcrate_durabilityimpl_140 {
() => {
// Module: crate::durability
// Provides: {"impl_140"}
// Dependencies: {}
impl std :: fmt :: Debug for Durability { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if f . alternate () { match self . 0 { DurabilityVal :: Low => f . write_str ("Durability::LOW") , DurabilityVal :: Medium => f . write_str ("Durability::MEDIUM") , DurabilityVal :: High => f . write_str ("Durability::HIGH") , } } else { f . debug_tuple ("Durability") . field (& (self . 0 as usize)) . finish () } } }
};
}
