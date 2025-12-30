// Generated macro for impl_270 (impl)
macro_rules! Depcrate_clientimpl_270 {
() => {
// Module: crate::client
// Provides: {"impl_270"}
// Dependencies: {}
impl ServerConnection { # [doc = " Create a new connection using `stream`."] pub fn new (conn : Box < dyn Connection >) -> io :: Result < ServerConnection > { let write_conn = conn . try_clone () ? ; Ok (ServerConnection { reader : BufReader :: new (conn) , writer : BufWriter :: new (write_conn) , }) } # [doc = " Send `request` to the server, read and return a `Response`."] pub fn request (& mut self , request : Request) -> Result < Response > { trace ! ("ServerConnection::request") ; util :: write_length_prefixed_bincode (& mut self . writer , request) ? ; trace ! ("ServerConnection::request: sent request") ; self . read_one_response () } # [doc = " Read a single `Response` from the server."] pub fn read_one_response (& mut self) -> Result < Response > { trace ! ("ServerConnection::read_one_response") ; let mut bytes = [0 ; 4] ; self . reader . read_exact (& mut bytes) . context ("Failed to read response header") ? ; let len = BigEndian :: read_u32 (& bytes) ; trace ! ("Should read {} more bytes" , len) ; let mut data = vec ! [0 ; len as usize] ; self . reader . read_exact (& mut data) ? ; trace ! ("Done reading") ; Ok (bincode :: deserialize (& data) ?) } }
};
}
