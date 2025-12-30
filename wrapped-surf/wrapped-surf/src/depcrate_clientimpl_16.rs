// Generated macro for impl_16 (impl)
macro_rules! Depcrate_clientimpl_16 {
() => {
// Module: crate::client
// Provides: {"impl_16"}
// Dependencies: {}
impl TryFrom < Config > for Client { # [cfg (feature = "default-client")] type Error = < DefaultClient as TryFrom < http_client :: Config > > :: Error ; # [cfg (not (feature = "default-client"))] type Error = std :: convert :: Infallible ; fn try_from (mut config : Config) -> std :: result :: Result < Self , Self :: Error > { let http_client = match config . http_client . take () { Some (client) => client , # [cfg (feature = "default-client")] None => Arc :: new (DefaultClient :: try_from (config . http_config . clone ()) ?) , # [cfg (not (feature = "default-client"))] None => panic ! ("Config without an http client provided to Surf configured without a default client.") } ; Ok (Client { config , http_client , middleware : Arc :: new (vec ! []) , }) } }
};
}
