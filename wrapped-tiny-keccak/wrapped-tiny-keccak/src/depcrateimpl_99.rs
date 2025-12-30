// Generated macro for impl_99 (impl)
macro_rules! Depcrateimpl_99 {
() => {
// Module: crate
// Provides: {"impl_99"}
// Dependencies: {}
impl < P > Clone for KeccakState < P > { fn clone (& self) -> Self { KeccakState { buffer : self . buffer . clone () , offset : self . offset , rate : self . rate , delim : self . delim , mode : self . mode , permutation : core :: marker :: PhantomData , } } }
};
}
