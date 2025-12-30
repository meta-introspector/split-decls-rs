// Generated macro for impl_43 (impl)
macro_rules! Depcrate_opsimpl_43 {
() => {
// Module: crate::ops
// Provides: {"impl_43"}
// Dependencies: {}
impl core :: ops :: Mul < usize > for LengthHint { type Output = Self ; fn mul (self , other : usize) -> Self { Self (self . 0 . saturating_mul (other) , self . 1 . and_then (| upper | upper . checked_mul (other)) ,) } }
};
}
