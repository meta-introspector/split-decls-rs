// Generated macro for solana_rpc_client (module)
macro_rules! Depcrate_example_mockssolana_rpc_client {
() => {
// Module: crate::example_mocks
// Provides: {"solana_rpc_client"}
// Dependencies: {}
pub mod solana_rpc_client { pub mod rpc_client { use { super :: super :: solana_rpc_client_api :: client_error :: Result as ClientResult , crate :: { hash :: Hash , signature :: Signature , transaction :: Transaction } , } ; pub struct RpcClient ; impl RpcClient { pub fn new (_url : String) -> Self { RpcClient } pub fn get_latest_blockhash (& self) -> ClientResult < Hash > { Ok (Hash :: default ()) } pub fn send_and_confirm_transaction (& self , _transaction : & Transaction ,) -> ClientResult < Signature > { Ok (Signature :: default ()) } } } }
};
}
