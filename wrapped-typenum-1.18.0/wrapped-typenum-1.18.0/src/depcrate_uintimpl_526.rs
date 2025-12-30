// Generated macro for impl_526 (impl)
macro_rules! Depcrate_uintimpl_526 {
() => {
// Module: crate::uint
// Provides: {"impl_526"}
// Dependencies: {}
impl < U , Ba , Bb > PrivateSquareRoot for UInt < UInt < U , Ba > , Bb > where U : Unsigned , Ba : Bit , Bb : Bit , U : SquareRoot , Sqrt < U > : Shl < B1 > , Double < Sqrt < U > > : Add < B1 > , Add1 < Double < Sqrt < U > > > : Mul , Self : IsGreaterOrEqual < Square < Add1 < Double < Sqrt < U > > > > > , Double < Sqrt < U > > : Add < GrEq < Self , Square < Add1 < Double < Sqrt < U > > > > > > , { type Output = Sum < Double < Sqrt < U > > , GrEq < Self , Square < Add1 < Double < Sqrt < U > > > > > > ; }
};
}
