// Generated macro for impl_357 (impl)
macro_rules! Depcrate_metrics_labelsimpl_357 {
() => {
// Module: crate::metrics::labels
// Provides: {"impl_357"}
// Dependencies: {}
impl std :: fmt :: Display for QuicInvalidInitialPacketError { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match self { Self :: FailedToParse => f . write_str ("failed to parse packet") , Self :: TokenValidationFail => f . write_str ("token validation fail") , Self :: WrongType (ty) => write ! (f , "wrong type: {ty:?}") , Self :: AcceptQueueOverflow => f . write_str ("accept queue overflow") , Self :: Unexpected => f . write_str ("unexpected error") , } } }
};
}
