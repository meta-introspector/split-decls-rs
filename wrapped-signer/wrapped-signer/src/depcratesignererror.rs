// Generated macro for SignerError (enum)
macro_rules! DepcrateSignerError {
() => {
// Module: crate
// Provides: {"SignerError"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq)] pub enum SignerError { KeypairPubkeyMismatch , NotEnoughSigners , TransactionError (TransactionError) , Custom (String) , PresignerError (PresignerError) , Connection (String) , InvalidInput (String) , NoDeviceFound , Protocol (String) , UserCancel (String) , TooManySigners , }
};
}
