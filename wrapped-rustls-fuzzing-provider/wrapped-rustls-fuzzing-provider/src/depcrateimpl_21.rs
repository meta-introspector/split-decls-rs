// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl crypto :: SecureRandom for Provider { fn fill (& self , bytes : & mut [u8]) -> Result < () , GetRandomFailed > { for (out , value) in bytes . iter_mut () . zip (RAND . iter () . cycle ()) { * out = * value ; } Ok (()) } }
};
}
