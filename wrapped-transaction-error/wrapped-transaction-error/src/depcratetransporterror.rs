// Generated macro for TransportError (enum)
macro_rules! DepcrateTransportError {
() => {
// Module: crate
// Provides: {"TransportError"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] # [derive (Debug)] pub enum TransportError { IoError (std :: io :: Error) , TransactionError (TransactionError) , Custom (String) , }
};
}
