// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [tokio :: main] async fn main () { tracing_subscriber :: fmt :: init () ; let config = Config :: parse () ; let addr = SocketAddr :: from (([0 , 0 , 0 , 0] , config . port)) ; match config . command { Command :: Server => { let listener = TcpListener :: bind (addr) . await . unwrap () ; serve_forever (listener) . await . expect ("server error") ; } Command :: Get { key } => { let mut client = make_client (addr) . await . unwrap () ; let result = client . get (GetRequest { key }) . await ; match result { Ok (response) => { let value_bytes = response . into_inner () . value ; let value = String :: from_utf8_lossy (& value_bytes [..]) ; print ! ("{}" , value) ; } Err (status) if status . code () == Code :: NotFound => { eprintln ! ("not found") ; std :: process :: exit (1) ; } Err (status) => { panic ! ("{:?}" , status) ; } } } Command :: Set { key } => { let mut client = make_client (addr) . await . unwrap () ; let mut stdin = tokio :: io :: stdin () ; let mut value = Vec :: new () ; stdin . read_to_end (& mut value) . await . unwrap () ; client . set (SetRequest { key , value }) . await . unwrap () ; println ! ("OK") ; } Command :: Subscribe => { let mut client = make_client (addr) . await . unwrap () ; let mut stream = client . subscribe (SubscribeRequest { }) . await . unwrap () . into_inner () ; println ! ("Stream created!") ; while let Some (item) = stream . next () . await { let item = item . unwrap () ; println ! ("key inserted: {:?}" , item . key) ; } } } }
};
}
