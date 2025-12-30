// Generated macro for impl_103 (impl)
macro_rules! Depcrate_yokeimpl_103 {
() => {
// Module: crate::yoke
// Provides: {"impl_103"}
// Dependencies: {}
impl < Y : for < 'a > Yokeable < 'a > , C : core :: fmt :: Debug > core :: fmt :: Debug for Yoke < Y , C > where for < 'a > < Y as Yokeable < 'a > > :: Output : core :: fmt :: Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("Yoke") . field ("yokeable" , self . get ()) . field ("cart" , self . backing_cart ()) . finish () } }
};
}
