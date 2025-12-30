// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl core :: error :: Error for SignerError { fn source (& self) -> :: core :: option :: Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: KeypairPubkeyMismatch => None , Self :: NotEnoughSigners => None , Self :: TransactionError (e) => Some (e) , Self :: Custom (_) => None , Self :: PresignerError (e) => Some (e) , Self :: Connection (_) => None , Self :: InvalidInput (_) => None , Self :: NoDeviceFound => None , Self :: Protocol (_) => None , Self :: UserCancel (_) => None , Self :: TooManySigners => None , } } }
};
}
