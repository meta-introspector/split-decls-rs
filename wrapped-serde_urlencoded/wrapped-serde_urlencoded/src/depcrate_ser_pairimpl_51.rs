// Generated macro for impl_51 (impl)
macro_rules! Depcrate_ser_pairimpl_51 {
() => {
// Module: crate::ser::pair
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'input , 'target , Target > PairSerializer < 'input , 'target , Target > where Target : 'target + UrlEncodedTarget , { pub fn new (urlencoder : & 'target mut UrlEncodedSerializer < 'input , Target > ,) -> Self { PairSerializer { urlencoder , state : PairState :: WaitingForKey , } } }
};
}
