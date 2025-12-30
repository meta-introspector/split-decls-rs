// Generated macro for make_pair (function)
macro_rules! Depcrate_common_test_streammake_pair {
() => {
// Module: crate::common::test_stream
// Provides: {"make_pair"}
// Dependencies: {}
fn make_pair () -> (ServerConnection , ClientConnection) { let (sconfig , cconfig) = utils :: make_configs () ; let server = ServerConnection :: new (Arc :: new (sconfig)) . unwrap () ; let domain = ServerName :: try_from ("foobar.com") . unwrap () ; let client = ClientConnection :: new (Arc :: new (cconfig) , domain) . unwrap () ; (server , client) }
};
}
