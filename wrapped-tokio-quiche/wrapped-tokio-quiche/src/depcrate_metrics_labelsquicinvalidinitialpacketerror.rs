// Generated macro for QuicInvalidInitialPacketError (enum)
macro_rules! Depcrate_metrics_labelsQuicInvalidInitialPacketError {
() => {
// Module: crate::metrics::labels
// Provides: {"QuicInvalidInitialPacketError"}
// Dependencies: {}
# [doc = " Reason why a QUIC Initial was discarded by the packet router."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum QuicInvalidInitialPacketError { TokenValidationFail , FailedToParse , WrongType (quiche :: Type) , AcceptQueueOverflow , Unexpected , }
};
}
