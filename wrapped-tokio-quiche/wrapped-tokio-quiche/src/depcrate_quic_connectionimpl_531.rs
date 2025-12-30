// Generated macro for impl_531 (impl)
macro_rules! Depcrate_quic_connectionimpl_531 {
() => {
// Module: crate::quic::connection
// Provides: {"impl_531"}
// Dependencies: {}
impl fmt :: Debug for QuicCommand { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Self :: ConnectionClose (b) => f . debug_tuple ("ConnectionClose") . field (b) . finish () , Self :: Custom (_) => f . debug_tuple ("Custom") . finish_non_exhaustive () , Self :: Stats (_) => f . debug_tuple ("Stats") . finish_non_exhaustive () , } } }
};
}
