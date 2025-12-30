// Generated macro for impl_385 (impl)
macro_rules! Depcrate_uintimpl_385 {
() => {
// Module: crate::uint
// Provides: {"impl_385"}
// Dependencies: {}
impl < U : Unsigned , B : Bit > core :: fmt :: Binary for UInt < UInt < U , B > , B1 > where UInt < U , B > : core :: fmt :: Binary , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{:b}1" , UInt ::< U , B >:: new ()) } }
};
}
