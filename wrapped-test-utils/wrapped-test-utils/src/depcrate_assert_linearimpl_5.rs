// Generated macro for impl_5 (impl)
macro_rules! Depcrate_assert_linearimpl_5 {
() => {
// Module: crate::assert_linear
// Provides: {"impl_5"}
// Dependencies: {}
impl AssertLinear { pub fn next_round (& mut self) -> bool { if let Some (round) = self . rounds . last_mut () { round . finish () ; } if self . rounds . iter () . any (| it | it . linear) || self . rounds . len () == 4 { return false ; } self . rounds . push (Round :: default ()) ; true } pub fn sample (& mut self , x : f64 , y : f64) { self . rounds . last_mut () . unwrap () . samples . push ((x , y)) ; } }
};
}
