// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < F , S , E > Test < S , E > for F where F : Fn (& std :: path :: Path) -> Result < S , E > , S : std :: fmt :: Display , E : std :: fmt :: Display , { fn run (& self , fixture : & std :: path :: Path) -> Result < S , E > { (self) (fixture) } }
};
}
