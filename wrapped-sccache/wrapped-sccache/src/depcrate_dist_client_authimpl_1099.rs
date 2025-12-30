// Generated macro for impl_1099 (impl)
macro_rules! Depcrate_dist_client_authimpl_1099 {
() => {
// Module: crate::dist::client_auth
// Provides: {"impl_1099"}
// Dependencies: {}
impl HyperBuilderWrap { pub async fn try_bind (addr : SocketAddr) -> io :: Result < HyperBuilderWrap > { let listener = TcpListener :: bind (addr) . await ? ; Ok (HyperBuilderWrap { listener }) } async fn serve < F > (& mut self , sfn : F) -> io :: Result < () > where F : Fn (hyper :: Request < hyper :: body :: Incoming >) -> anyhow :: Result < Response < Full < Bytes > > > + Send + 'static + Copy + Sync , { use hyper :: server :: conn :: http1 ; use hyper_util :: rt :: tokio :: TokioIo ; loop { let (tcp , _) = self . listener . accept () . await ? ; let io = TokioIo :: new (tcp) ; tokio :: task :: spawn (async move { let conn = http1 :: Builder :: new () . serve_connection (io , hyper :: service :: service_fn (| req | async move { let uri = req . uri () . clone () ; sfn (req) . or_else (| e | error_code_response (uri , e)) }) ,) ; tokio :: pin ! (conn) ; conn . await . unwrap () ; }) ; } } pub fn local_addr (& self) -> SocketAddr { self . listener . local_addr () . unwrap () } }
};
}
