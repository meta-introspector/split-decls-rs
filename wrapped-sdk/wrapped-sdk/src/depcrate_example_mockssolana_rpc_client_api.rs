// Generated macro for solana_rpc_client_api (module)
macro_rules! Depcrate_example_mockssolana_rpc_client_api {
() => {
// Module: crate::example_mocks
// Provides: {"solana_rpc_client_api"}
// Dependencies: {}
pub mod solana_rpc_client_api { pub mod client_error { use thiserror :: Error ; # [derive (Error , Debug)] # [error ("mock-error")] pub struct ClientError ; pub type Result < T > = std :: result :: Result < T , ClientError > ; } }
};
}
