// Generated macro for impl_38 (impl)
macro_rules! Depcrate_opsimpl_38 {
() => {
// Module: crate::ops
// Provides: {"impl_38"}
// Dependencies: {}
impl core :: ops :: Add < LengthHint > for LengthHint { type Output = Self ; fn add (self , other : LengthHint) -> Self { LengthHint (self . 0 . saturating_add (other . 0) , match (self . 1 , other . 1) { (Some (c) , Some (d)) => c . checked_add (d) , _ => None , } ,) } }
};
}
