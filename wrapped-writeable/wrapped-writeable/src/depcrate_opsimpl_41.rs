// Generated macro for impl_41 (impl)
macro_rules! Depcrate_opsimpl_41 {
() => {
// Module: crate::ops
// Provides: {"impl_41"}
// Dependencies: {}
impl core :: ops :: Add < usize > for LengthHint { type Output = Self ; fn add (self , other : usize) -> Self { Self (self . 0 . saturating_add (other) , self . 1 . and_then (| upper | upper . checked_add (other)) ,) } }
};
}
