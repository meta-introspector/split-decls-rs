// Generated macro for connect_pinned (function)
macro_rules! Depcrateconnect_pinned {
() => {
// Module: crate
// Provides: {"connect_pinned"}
// Dependencies: {}
# [doc = " Make a TCP connection to `addr` and set up a TLS session."] # [doc = ""] # [doc = " The first time you try to write or read the returned stream,"] # [doc = " `rustls` will do TLS negotiation."] # [doc = " TLS negotiation fails if the server provides a leaf cert"] # [doc = " that is not in `certs`."] # [doc = ""] # [doc = " Ignores hostnames in certificates."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to open the TCP connection."] # [doc = ""] # [doc = " # Example"] # [doc = " See example in [`rustls_pin`](index.html) crate docs."] pub fn connect_pinned (addr : impl ToSocketAddrs , certs : impl AsRef < [rustls :: Certificate] > + Send + Sync + 'static ,) -> Result < rustls :: StreamOwned < ClientSession , TcpStream > , std :: io :: Error > { let tcp_stream = std :: net :: TcpStream :: connect (addr) ? ; let mut client_config = rustls :: ClientConfig :: new () ; client_config . dangerous () . set_certificate_verifier (Arc :: new (PinnedServerCertVerifier :: new (certs))) ; let session = rustls :: ClientSession :: new (& Arc :: new (client_config) , arbitrary_dns_name () . as_ref ()) ; Ok (rustls :: StreamOwned :: new (session , tcp_stream)) }
};
}
