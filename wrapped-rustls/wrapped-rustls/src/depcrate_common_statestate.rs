// Generated macro for State (trait)
macro_rules! Depcrate_common_stateState {
() => {
// Module: crate::common_state
// Provides: {"State"}
// Dependencies: {}
pub (crate) trait State < Side > : Send + Sync { fn handle < 'm > (self : Box < Self > , cx : & mut Context < '_ , Side > , message : Message < 'm > ,) -> Result < Box < dyn State < Side > > , Error > ; fn send_key_update_request (& mut self , _common : & mut CommonState) -> Result < () , Error > { Err (Error :: HandshakeNotComplete) } fn handle_decrypt_error (& self) { } fn into_external_state (self : Box < Self > ,) -> Result < (PartiallyExtractedSecrets , Box < dyn KernelState + 'static >) , Error > { Err (Error :: HandshakeNotComplete) } }
};
}
