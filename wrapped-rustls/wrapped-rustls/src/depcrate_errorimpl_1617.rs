// Generated macro for impl_1617 (impl)
macro_rules! Depcrate_errorimpl_1617 {
() => {
// Module: crate::error
// Provides: {"impl_1617"}
// Dependencies: {}
impl fmt :: Display for ExtendedKeyPurpose { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: ClientAuth => write ! (f , "client authentication") , Self :: ServerAuth => write ! (f , "server authentication") , Self :: Other (values) => { for (i , value) in values . iter () . enumerate () { if i > 0 { write ! (f , ", ") ? ; } write ! (f , "{value}") ? ; } Ok (()) } } } }
};
}
