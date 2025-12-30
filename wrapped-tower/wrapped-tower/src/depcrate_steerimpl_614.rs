// Generated macro for impl_614 (impl)
macro_rules! Depcrate_steerimpl_614 {
() => {
// Module: crate::steer
// Provides: {"impl_614"}
// Dependencies: {}
impl < S , F , Req > Picker < S , Req > for F where F : Fn (& Req , & [S]) -> usize , { fn pick (& mut self , r : & Req , services : & [S]) -> usize { self (r , services) } }
};
}
