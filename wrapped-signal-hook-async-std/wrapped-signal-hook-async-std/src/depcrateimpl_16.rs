// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl < E : Exfiltrator > Stream for SignalsInfo < E > { type Item = E :: Output ; fn poll_next (mut self : Pin < & mut Self > , ctx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match self . 0 . poll_signal (& mut | read | Self :: has_signals (read , ctx)) { PollResult :: Signal (sig) => Poll :: Ready (Some (sig)) , PollResult :: Closed => Poll :: Ready (None) , PollResult :: Pending => Poll :: Pending , PollResult :: Err (error) => panic ! ("Unexpected error: {}" , error) , } } }
};
}
