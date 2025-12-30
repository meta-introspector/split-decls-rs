// Generated macro for impl_407 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_407 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_407"}
// Dependencies: {}
impl DuplicateExtensionChecker { fn new () -> Self { Self (BTreeSet :: new ()) } fn check (& mut self , typ : ExtensionType) -> Result < () , InvalidMessage > { let u = u16 :: from (typ) ; match self . 0 . insert (u) { true => Ok (()) , false => Err (InvalidMessage :: DuplicateExtension (u)) , } } }
};
}
