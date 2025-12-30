// Generated macro for impl_66 (impl)
macro_rules! Depcrate_intimpl_66 {
() => {
// Module: crate::int
// Provides: {"impl_66"}
// Dependencies: {}
impl < U : Unsigned + NonZero + core :: fmt :: Binary > core :: fmt :: Binary for NInt < U > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "-{:b}" , self . n) } }
};
}
