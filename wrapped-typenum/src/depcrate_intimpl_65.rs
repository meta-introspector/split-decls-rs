// Generated macro for impl_65 (impl)
macro_rules! Depcrate_intimpl_65 {
() => {
// Module: crate::int
// Provides: {"impl_65"}
// Dependencies: {}
impl < U : Unsigned + NonZero + core :: fmt :: Binary > core :: fmt :: Binary for PInt < U > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "+{:b}" , self . n) } }
};
}
