// Generated macro for impl_384 (impl)
macro_rules! Depcrate_uintimpl_384 {
() => {
// Module: crate::uint
// Provides: {"impl_384"}
// Dependencies: {}
impl < U : Unsigned , B : Bit > core :: fmt :: Binary for UInt < UInt < U , B > , B0 > where UInt < U , B > : core :: fmt :: Binary , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{:b}0" , UInt ::< U , B >:: new ()) } }
};
}
