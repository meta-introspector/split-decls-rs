// Generated macro for impl_142 (impl)
macro_rules! Depcrate_durabilityimpl_142 {
() => {
// Module: crate::durability
// Provides: {"impl_142"}
// Dependencies: {}
impl From < u8 > for DurabilityVal { fn from (value : u8) -> Self { match value { 0 => DurabilityVal :: Low , 1 => DurabilityVal :: Medium , 2 => DurabilityVal :: High , _ => panic ! ("invalid durability") , } } }
};
}
