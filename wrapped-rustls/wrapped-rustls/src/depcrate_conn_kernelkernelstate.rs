// Generated macro for KernelState (trait)
macro_rules! Depcrate_conn_kernelKernelState {
() => {
// Module: crate::conn::kernel
// Provides: {"KernelState"}
// Dependencies: {}
pub (crate) trait KernelState : Send + Sync { # [doc = " Update the traffic secret for the specified direction on the connection."] fn update_secrets (& mut self , dir : Direction) -> Result < ConnectionTrafficSecrets , Error > ; # [doc = " Handle a new session ticket."] # [doc = ""] # [doc = " This will only ever be called for client connections, as [`KernelConnection`]"] # [doc = " only exposes the relevant API for client connections."] fn handle_new_session_ticket (& mut self , cx : & mut KernelContext < '_ > , message : & NewSessionTicketPayloadTls13 ,) -> Result < () , Error > ; }
};
}
